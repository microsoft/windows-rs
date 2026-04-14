
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
#ifndef __windows2Eservices2Etargetedcontent_h__
#define __windows2Eservices2Etargetedcontent_h__
#ifndef __windows2Eservices2Etargetedcontent_p_h__
#define __windows2Eservices2Etargetedcontent_p_h__


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

#if !defined(WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION)
#define WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentAction;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction ABI::Windows::Services::TargetedContent::ITargetedContentAction

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentAvailabilityChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs ABI::Windows::Services::TargetedContent::ITargetedContentAvailabilityChangedEventArgs

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs ABI::Windows::Services::TargetedContent::ITargetedContentChangedEventArgs

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentCollection;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection ABI::Windows::Services::TargetedContent::ITargetedContentCollection

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentContainer;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer ABI::Windows::Services::TargetedContent::ITargetedContentContainer

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentContainerStatics;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics ABI::Windows::Services::TargetedContent::ITargetedContentContainerStatics

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentImage;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage ABI::Windows::Services::TargetedContent::ITargetedContentImage

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentItem;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem ABI::Windows::Services::TargetedContent::ITargetedContentItem

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentItemState;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState ABI::Windows::Services::TargetedContent::ITargetedContentItemState

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentObject;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject ABI::Windows::Services::TargetedContent::ITargetedContentObject

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentStateChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs ABI::Windows::Services::TargetedContent::ITargetedContentStateChangedEventArgs

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentSubscription;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription ABI::Windows::Services::TargetedContent::ITargetedContentSubscription

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentSubscriptionOptions;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions ABI::Windows::Services::TargetedContent::ITargetedContentSubscriptionOptions

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentSubscriptionStatics;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics ABI::Windows::Services::TargetedContent::ITargetedContentSubscriptionStatics

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                interface ITargetedContentValue;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue ABI::Windows::Services::TargetedContent::ITargetedContentValue

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentContainer;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e757e0fc-0136-5f63-97b8-6a96b8d0601e"))
IAsyncOperation<ABI::Windows::Services::TargetedContent::TargetedContentContainer*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentContainer*, ABI::Windows::Services::TargetedContent::ITargetedContentContainer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.TargetedContent.TargetedContentContainer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::TargetedContent::TargetedContentContainer*> __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_t;
#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8fc6bc2a-26ce-50b5-97bb-fcc80ca0871d"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::TargetedContent::TargetedContentContainer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentContainer*, ABI::Windows::Services::TargetedContent::ITargetedContentContainer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.TargetedContent.TargetedContentContainer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::TargetedContent::TargetedContentContainer*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentSubscription;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("46f16f4b-8ec1-5c4f-b1f5-a7e7acd63366"))
IAsyncOperation<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::ITargetedContentSubscription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.TargetedContent.TargetedContentSubscription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*> __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_t;
#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e4188c71-5a8e-57ec-b0de-1d314fb3e2cf"))
IAsyncOperationCompletedHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::ITargetedContentSubscription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.TargetedContent.TargetedContentSubscription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_boolean_USE
#define DEF___FIIterator_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("740a0296-a535-572a-bf0b-17c18ff71fe6"))
IIterator<bool> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<bool> __FIIterator_1_boolean_t;
#define __FIIterator_1_boolean ABI::Windows::Foundation::Collections::__FIIterator_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_boolean_USE */



#ifndef DEF___FIIterable_1_boolean_USE
#define DEF___FIIterable_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("30160817-1d7d-54e9-99db-d7636266a476"))
IIterable<bool> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<bool> __FIIterable_1_boolean_t;
#define __FIIterable_1_boolean ABI::Windows::Foundation::Collections::__FIIterable_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_boolean_USE */



#ifndef DEF___FIIterator_1_double_USE
#define DEF___FIIterator_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("638a2cf4-f474-5318-9055-141cb909ac4b"))
IIterator<double> : IIterator_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<double> __FIIterator_1_double_t;
#define __FIIterator_1_double ABI::Windows::Foundation::Collections::__FIIterator_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_double_USE */



#ifndef DEF___FIIterable_1_double_USE
#define DEF___FIIterable_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c738964e-9c64-5bce-b5ce-61e9a282ec4a"))
IIterable<double> : IIterable_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<double> __FIIterable_1_double_t;
#define __FIIterable_1_double ABI::Windows::Foundation::Collections::__FIIterable_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_double_USE */



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



#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentValue;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("35cf9903-ade5-565d-a011-be3173d09215"))
IKeyValuePair<HSTRING, ABI::Windows::Services::TargetedContent::TargetedContentValue*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentValue*, ABI::Windows::Services::TargetedContent::ITargetedContentValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Services.TargetedContent.TargetedContentValue>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Services::TargetedContent::TargetedContentValue*> __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b97e682b-6e0a-5eea-b70b-25795b28e937"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Services.TargetedContent.TargetedContentValue>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("45a020d8-fe49-5720-950b-3cceab655531"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Services.TargetedContent.TargetedContentValue>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c157d0f-5efe-5cec-bbd6-0c6ce9af07a5"))
IIterator<ABI::Windows::Foundation::Uri*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Uri*> __FIIterator_1_Windows__CFoundation__CUri_t;
#define __FIIterator_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0d63b78-78ad-5e31-b6d8-e32a0e16c447"))
IIterable<ABI::Windows::Foundation::Uri*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Uri*> __FIIterable_1_Windows__CFoundation__CUri_t;
#define __FIIterable_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentAction;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#define DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("79656935-5813-5aa6-8e69-627a0d85088f"))
IIterator<ABI::Windows::Services::TargetedContent::TargetedContentAction*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentAction*, ABI::Windows::Services::TargetedContent::ITargetedContentAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.TargetedContent.TargetedContentAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::TargetedContent::TargetedContentAction*> __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t;
#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#define DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf05b497-3afd-5d00-859e-9fbd1a36d576"))
IIterable<ABI::Windows::Services::TargetedContent::TargetedContentAction*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentAction*, ABI::Windows::Services::TargetedContent::ITargetedContentAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.TargetedContent.TargetedContentAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::TargetedContent::TargetedContentAction*> __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t;
#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentCollection;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#define DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6093b8fd-6d5d-53cd-b497-7b4540f10857"))
IIterator<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentCollection*, ABI::Windows::Services::TargetedContent::ITargetedContentCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.TargetedContent.TargetedContentCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t;
#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#define DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2049f813-37ee-5158-9996-709859f0ce49"))
IIterable<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentCollection*, ABI::Windows::Services::TargetedContent::ITargetedContentCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.TargetedContent.TargetedContentCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t;
#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentFile;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

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

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#define DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a957f20-ed25-5019-90e7-9890d4f912f2"))
IIterator<ABI::Windows::Services::TargetedContent::TargetedContentFile*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentFile*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.TargetedContent.TargetedContentFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::TargetedContent::TargetedContentFile*> __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t;
#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#define DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5f65d649-ccbd-5728-a85b-d3ff92fca962"))
IIterable<ABI::Windows::Services::TargetedContent::TargetedContentFile*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentFile*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.TargetedContent.TargetedContentFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::TargetedContent::TargetedContentFile*> __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t;
#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentImage;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#define DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a807b298-9e2f-5673-bcf6-1e35feba0647"))
IIterator<ABI::Windows::Services::TargetedContent::TargetedContentImage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentImage*, ABI::Windows::Services::TargetedContent::ITargetedContentImage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.TargetedContent.TargetedContentImage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::TargetedContent::TargetedContentImage*> __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t;
#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#define DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("efadb6bf-af18-5af9-a509-19881bc586f5"))
IIterable<ABI::Windows::Services::TargetedContent::TargetedContentImage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentImage*, ABI::Windows::Services::TargetedContent::ITargetedContentImage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.TargetedContent.TargetedContentImage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::TargetedContent::TargetedContentImage*> __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t;
#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentItem;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#define DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("50109d8e-f711-5076-8309-e4e230ef7e85"))
IIterator<ABI::Windows::Services::TargetedContent::TargetedContentItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentItem*, ABI::Windows::Services::TargetedContent::ITargetedContentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.TargetedContent.TargetedContentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Services::TargetedContent::TargetedContentItem*> __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t;
#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#define DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("03f38fb6-54e6-5bf1-913b-9510fec8be1f"))
IIterable<ABI::Windows::Services::TargetedContent::TargetedContentItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentItem*, ABI::Windows::Services::TargetedContent::ITargetedContentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.TargetedContent.TargetedContentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Services::TargetedContent::TargetedContentItem*> __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t;
#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */


#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#define DEF___FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("19a87e94-ab75-574f-a226-8726a0d8eb3e"))
IMapView<HSTRING, ABI::Windows::Services::TargetedContent::TargetedContentValue*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentValue*, ABI::Windows::Services::TargetedContent::ITargetedContentValue*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Services.TargetedContent.TargetedContentValue>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Services::TargetedContent::TargetedContentValue*> __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t;
#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIVectorView_1_boolean_USE
#define DEF___FIVectorView_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("243a09cb-6f40-56af-a442-fe81431fbef5"))
IVectorView<bool> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<bool> __FIVectorView_1_boolean_t;
#define __FIVectorView_1_boolean ABI::Windows::Foundation::Collections::__FIVectorView_1_boolean_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_boolean_USE */



#ifndef DEF___FIVectorView_1_double_USE
#define DEF___FIVectorView_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af7586a8-6b21-5f61-bff1-1b682293ad96"))
IVectorView<double> : IVectorView_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<double> __FIVectorView_1_double_t;
#define __FIVectorView_1_double ABI::Windows::Foundation::Collections::__FIVectorView_1_double_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_double_USE */



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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b8385bd-a2cd-5ff1-bf74-7ea580423e50"))
IVectorView<ABI::Windows::Foundation::Uri*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::Uri*> __FIVectorView_1_Windows__CFoundation__CUri_t;
#define __FIVectorView_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#define DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4299bd84-e44e-5fcb-a465-e1bd434a317c"))
IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentAction*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentAction*, ABI::Windows::Services::TargetedContent::ITargetedContentAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.TargetedContent.TargetedContentAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentAction*> __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t;
#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#define DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cea4c859-8736-5c75-bb83-a686bf7f7c6f"))
IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentCollection*, ABI::Windows::Services::TargetedContent::ITargetedContentCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.TargetedContent.TargetedContentCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentCollection*> __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t;
#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#define DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ec0d80cb-9a87-5f0b-b6df-2c09b6310177"))
IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentFile*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentFile*, ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.TargetedContent.TargetedContentFile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentFile*> __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t;
#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#define DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f55ac7c6-168d-5010-84cf-36bf451ede38"))
IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentImage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentImage*, ABI::Windows::Services::TargetedContent::ITargetedContentImage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.TargetedContent.TargetedContentImage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentImage*> __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t;
#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#define DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("31e3ed33-8554-5496-86a4-d78392204c8f"))
IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentItem*, ABI::Windows::Services::TargetedContent::ITargetedContentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Services.TargetedContent.TargetedContentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Services::TargetedContent::TargetedContentItem*> __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t;
#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentAvailabilityChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("99929904-138a-59ac-a11a-fe0042f0fd50"))
ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentAvailabilityChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::ITargetedContentSubscription*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentAvailabilityChangedEventArgs*, ABI::Windows::Services::TargetedContent::ITargetedContentAvailabilityChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.TargetedContent.TargetedContentSubscription, Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentAvailabilityChangedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef11d751-9d56-580d-8a9f-51ae7e8036e3"))
ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::ITargetedContentSubscription*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentChangedEventArgs*, ABI::Windows::Services::TargetedContent::ITargetedContentChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.TargetedContent.TargetedContentSubscription, Windows.Services.TargetedContent.TargetedContentChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentChangedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentStateChangedEventArgs;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4d5acbe-f65b-5fa4-9242-d2860de85d52"))
ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::ITargetedContentSubscription*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Services::TargetedContent::TargetedContentStateChangedEventArgs*, ABI::Windows::Services::TargetedContent::ITargetedContentStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Services.TargetedContent.TargetedContentSubscription, Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Services::TargetedContent::TargetedContentSubscription*, ABI::Windows::Services::TargetedContent::TargetedContentStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_USE */

#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

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
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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
        namespace Services {
            namespace TargetedContent {
                typedef enum TargetedContentAppInstallationState : int TargetedContentAppInstallationState;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                typedef enum TargetedContentAvailability : int TargetedContentAvailability;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                typedef enum TargetedContentInteraction : int TargetedContentInteraction;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                typedef enum TargetedContentObjectKind : int TargetedContentObjectKind;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                typedef enum TargetedContentValueKind : int TargetedContentValueKind;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentItemState;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentObject;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                class TargetedContentSubscriptionOptions;
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentAppInstallationState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                enum TargetedContentAppInstallationState : int
                {
                    TargetedContentAppInstallationState_NotApplicable = 0,
                    TargetedContentAppInstallationState_NotInstalled = 1,
                    TargetedContentAppInstallationState_Installed = 2,
                };
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentAvailability
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                enum TargetedContentAvailability : int
                {
                    TargetedContentAvailability_None = 0,
                    TargetedContentAvailability_Partial = 1,
                    TargetedContentAvailability_All = 2,
                };
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentInteraction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                enum TargetedContentInteraction : int
                {
                    TargetedContentInteraction_Impression = 0,
                    TargetedContentInteraction_ClickThrough = 1,
                    TargetedContentInteraction_Hover = 2,
                    TargetedContentInteraction_Like = 3,
                    TargetedContentInteraction_Dislike = 4,
                    TargetedContentInteraction_Dismiss = 5,
                    TargetedContentInteraction_Ineligible = 6,
                    TargetedContentInteraction_Accept = 7,
                    TargetedContentInteraction_Decline = 8,
                    TargetedContentInteraction_Defer = 9,
                    TargetedContentInteraction_Canceled = 10,
                    TargetedContentInteraction_Conversion = 11,
                    TargetedContentInteraction_Opportunity = 12,
                };
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentObjectKind
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                enum TargetedContentObjectKind : int
                {
                    TargetedContentObjectKind_Collection = 0,
                    TargetedContentObjectKind_Item = 1,
                    TargetedContentObjectKind_Value = 2,
                };
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentValueKind
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                enum TargetedContentValueKind : int
                {
                    TargetedContentValueKind_String = 0,
                    TargetedContentValueKind_Uri = 1,
                    TargetedContentValueKind_Number = 2,
                    TargetedContentValueKind_Boolean = 3,
                    TargetedContentValueKind_File = 4,
                    TargetedContentValueKind_ImageFile = 5,
                    TargetedContentValueKind_Action = 6,
                    TargetedContentValueKind_Strings = 7,
                    TargetedContentValueKind_Uris = 8,
                    TargetedContentValueKind_Numbers = 9,
                    TargetedContentValueKind_Booleans = 10,
                    TargetedContentValueKind_Files = 11,
                    TargetedContentValueKind_ImageFiles = 12,
                    TargetedContentValueKind_Actions = 13,
                };
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentAction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentAction
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentAction[] = L"Windows.Services.TargetedContent.ITargetedContentAction";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("d75b691e-6cd6-4ca0-9d8f-4728b0b7e6b6")
                ITargetedContentAction : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE InvokeAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentAction = __uuidof(ITargetedContentAction);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentAvailabilityChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("e0f59d26-5927-4450-965c-1ceb7becde65")
                ITargetedContentAvailabilityChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentAvailabilityChangedEventArgs = __uuidof(ITargetedContentAvailabilityChangedEventArgs);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("99d488c9-587e-4586-8ef7-b54ca9453a16")
                ITargetedContentChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasPreviousContentExpired(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentChangedEventArgs = __uuidof(ITargetedContentChangedEventArgs);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentCollection
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentCollection
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentCollection[] = L"Windows.Services.TargetedContent.ITargetedContentCollection";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("2d4b66c5-f163-44ba-9f6e-e1a4c2bb559d")
                ITargetedContentCollection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportInteraction(
                        ABI::Windows::Services::TargetedContent::TargetedContentInteraction interaction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportCustomInteraction(
                        HSTRING customInteractionName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Collections(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Items(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentCollection = __uuidof(ITargetedContentCollection);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentContainer
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentContainer
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentContainer[] = L"Windows.Services.TargetedContent.ITargetedContentContainer";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("bc2494c9-8837-47c2-850f-d79d64595926")
                ITargetedContentContainer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Availability(
                        ABI::Windows::Services::TargetedContent::TargetedContentAvailability* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Services::TargetedContent::ITargetedContentCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SelectSingleObject(
                        HSTRING path,
                        ABI::Windows::Services::TargetedContent::ITargetedContentObject** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentContainer = __uuidof(ITargetedContentContainer);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentContainerStatics
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentContainer
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentContainerStatics[] = L"Windows.Services.TargetedContent.ITargetedContentContainerStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("5b47e7fb-2140-4c1f-a736-c59583f227d8")
                ITargetedContentContainerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsync(
                        HSTRING contentId,
                        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer** asyncOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentContainerStatics = __uuidof(ITargetedContentContainerStatics);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentImage
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentImage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStreamReference
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentImage[] = L"Windows.Services.TargetedContent.ITargetedContentImage";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("a7a585d9-779f-4b1e-bbb1-8eaf53fbeab2")
                ITargetedContentImage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentImage = __uuidof(ITargetedContentImage);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentItem
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentItem
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentItem[] = L"Windows.Services.TargetedContent.ITargetedContentItem";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("38168dc4-276c-4c32-96ba-565c6e406e74")
                ITargetedContentItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportInteraction(
                        ABI::Windows::Services::TargetedContent::TargetedContentInteraction interaction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportCustomInteraction(
                        HSTRING customInteractionName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Services::TargetedContent::ITargetedContentItemState** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Collections(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentItem = __uuidof(ITargetedContentItem);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentItemState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentItemState
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentItemState[] = L"Windows.Services.TargetedContent.ITargetedContentItemState";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("73935454-4c65-4b47-a441-472de53c79b6")
                ITargetedContentItemState : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldDisplay(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppInstallationState(
                        ABI::Windows::Services::TargetedContent::TargetedContentAppInstallationState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentItemState = __uuidof(ITargetedContentItemState);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentObject
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentObject
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentObject[] = L"Windows.Services.TargetedContent.ITargetedContentObject";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("041d7969-2212-42d1-9dfa-88a8e3033aa3")
                ITargetedContentObject : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ObjectKind(
                        ABI::Windows::Services::TargetedContent::TargetedContentObjectKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Collection(
                        ABI::Windows::Services::TargetedContent::ITargetedContentCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Item(
                        ABI::Windows::Services::TargetedContent::ITargetedContentItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Services::TargetedContent::ITargetedContentValue** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentObject = __uuidof(ITargetedContentObject);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentStateChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("9a1cef3d-8073-4416-8df2-546835a6414f")
                ITargetedContentStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentStateChangedEventArgs = __uuidof(ITargetedContentStateChangedEventArgs);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscription
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscription
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscription[] = L"Windows.Services.TargetedContent.ITargetedContentSubscription";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("882c2c49-c652-4c7a-acad-1f7fa2986c73")
                ITargetedContentSubscription : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetContentContainerAsync(
                        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ContentChanged(
                        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ContentChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AvailabilityChanged(
                        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AvailabilityChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentSubscription = __uuidof(ITargetedContentSubscription);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscriptionOptions
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscriptionOptions[] = L"Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("61ee6ad0-2c83-421b-8467-413eaf1aeb97")
                ITargetedContentSubscriptionOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SubscriptionId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowPartialContentAvailability(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowPartialContentAvailability(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CloudQueryParameters(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalFilters(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Update(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentSubscriptionOptions = __uuidof(ITargetedContentSubscriptionOptions);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscription
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscriptionStatics[] = L"Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("faddfe80-360d-4916-b53c-7ea27090d02a")
                ITargetedContentSubscriptionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAsync(
                        HSTRING subscriptionId,
                        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOptions(
                        HSTRING subscriptionId,
                        ABI::Windows::Services::TargetedContent::ITargetedContentSubscriptionOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentSubscriptionStatics = __uuidof(ITargetedContentSubscriptionStatics);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentValue
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentValue
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentValue[] = L"Windows.Services.TargetedContent.ITargetedContentValue";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace TargetedContent {
                MIDL_INTERFACE("aafde4b3-4215-4bf8-867f-43f04865f9bf")
                ITargetedContentValue : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ValueKind(
                        ABI::Windows::Services::TargetedContent::TargetedContentValueKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_String(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Number(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Boolean(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageFile(
                        ABI::Windows::Services::TargetedContent::ITargetedContentImage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        ABI::Windows::Services::TargetedContent::ITargetedContentAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Strings(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uris(
                        __FIVectorView_1_Windows__CFoundation__CUri** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Numbers(
                        __FIVectorView_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Booleans(
                        __FIVectorView_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Files(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageFiles(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Actions(
                        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITargetedContentValue = __uuidof(ITargetedContentValue);
            } /* TargetedContent */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentAction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentAction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAction_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentAction[] = L"Windows.Services.TargetedContent.TargetedContentAction";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentCollection
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentCollection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentCollection_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentCollection[] = L"Windows.Services.TargetedContent.TargetedContentCollection";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentContainer
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.TargetedContent.ITargetedContentContainerStatics interface starting with version 1.0 of the Windows.Services.TargetedContent.TargetedContentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentContainer_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentContainer[] = L"Windows.Services.TargetedContent.TargetedContentContainer";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentFile
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentFile_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentFile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentFile[] = L"Windows.Services.TargetedContent.TargetedContentFile";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentImage
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentImage ** Default Interface **
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentImage_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentImage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentImage[] = L"Windows.Services.TargetedContent.TargetedContentImage";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentItem
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItem_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentItem[] = L"Windows.Services.TargetedContent.TargetedContentItem";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentItemState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentItemState ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItemState_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItemState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentItemState[] = L"Windows.Services.TargetedContent.TargetedContentItemState";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentObject
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentObject ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentObject_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentObject[] = L"Windows.Services.TargetedContent.TargetedContentObject";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentSubscription
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics interface starting with version 1.0 of the Windows.Services.TargetedContent.TargetedContentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentSubscription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscription_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentSubscription[] = L"Windows.Services.TargetedContent.TargetedContentSubscription";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentSubscriptionOptions
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions[] = L"Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentValue
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentValue_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentValue[] = L"Windows.Services.TargetedContent.TargetedContentValue";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue;

#endif // ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer;

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer;

typedef struct __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* This,
        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentContainer_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription;

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription;

typedef struct __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* This,
        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_boolean_INTERFACE_DEFINED__)
#define ____FIIterator_1_boolean_INTERFACE_DEFINED__

typedef interface __FIIterator_1_boolean __FIIterator_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_boolean;

typedef struct __FIIterator_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_boolean* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_boolean* This,
        UINT32 itemsLength,
        boolean* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_booleanVtbl;

interface __FIIterator_1_boolean
{
    CONST_VTBL struct __FIIterator_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_boolean_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_boolean_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_boolean_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_boolean_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_boolean_INTERFACE_DEFINED__)
#define ____FIIterable_1_boolean_INTERFACE_DEFINED__

typedef interface __FIIterable_1_boolean __FIIterable_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_boolean;

typedef struct __FIIterable_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_boolean* This,
        __FIIterator_1_boolean** result);

    END_INTERFACE
} __FIIterable_1_booleanVtbl;

interface __FIIterable_1_boolean
{
    CONST_VTBL struct __FIIterable_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_boolean_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_double_INTERFACE_DEFINED__)
#define ____FIIterator_1_double_INTERFACE_DEFINED__

typedef interface __FIIterator_1_double __FIIterator_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_double;

typedef struct __FIIterator_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_double* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_double* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_double* This,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_doubleVtbl;

interface __FIIterator_1_double
{
    CONST_VTBL struct __FIIterator_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_double_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_double_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_double_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_double_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_double_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_double_INTERFACE_DEFINED__)
#define ____FIIterable_1_double_INTERFACE_DEFINED__

typedef interface __FIIterable_1_double __FIIterable_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_double;

typedef struct __FIIterable_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_double* This,
        __FIIterator_1_double** result);

    END_INTERFACE
} __FIIterable_1_doubleVtbl;

interface __FIIterable_1_double
{
    CONST_VTBL struct __FIIterable_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_double_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_double_INTERFACE_DEFINED__

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

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CUri __FIIterator_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CUri;

typedef struct __FIIterator_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CUriVtbl;

interface __FIIterator_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CUri_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CUri __FIIterable_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CUri;

typedef struct __FIIterable_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CUri* This,
        __FIIterator_1_Windows__CFoundation__CUri** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CUriVtbl;

interface __FIIterable_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CUri_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

typedef struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl;

interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

typedef struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentAction** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl;

interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

typedef struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl;

interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

typedef struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentCollection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl;

interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

typedef struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl;

interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

typedef struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentFile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl;

interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

typedef struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl;

interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

typedef struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentImage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl;

interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

typedef struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl;

interface __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

typedef struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        __FIIterator_1_Windows__CServices__CTargetedContent__CTargetedContentItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl;

interface __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue;

typedef struct __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING key,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue* This,
        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** first,
        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl;

interface __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_boolean_INTERFACE_DEFINED__)
#define ____FIVectorView_1_boolean_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_boolean __FIVectorView_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_boolean;

typedef struct __FIVectorView_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_boolean* This,
        UINT32 index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_boolean* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_boolean* This,
        boolean value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_boolean* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        boolean* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_booleanVtbl;

interface __FIVectorView_1_boolean
{
    CONST_VTBL struct __FIVectorView_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_boolean_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_boolean_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_boolean_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_boolean_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_double_INTERFACE_DEFINED__)
#define ____FIVectorView_1_double_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_double __FIVectorView_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_double;

typedef struct __FIVectorView_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_double* This,
        UINT32 index,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_double* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_double* This,
        DOUBLE value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_double* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        DOUBLE* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_doubleVtbl;

interface __FIVectorView_1_double
{
    CONST_VTBL struct __FIVectorView_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_double_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_double_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_double_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_double_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_double_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CUri __FIVectorView_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CUri;

typedef struct __FIVectorView_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CUriVtbl;

interface __FIVectorView_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction;

typedef struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl;

interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection;

typedef struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl;

interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile;

typedef struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        UINT32 index,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl;

interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage;

typedef struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl;

interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem;

typedef struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        UINT32 index,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl;

interface __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* sender,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* sender,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* sender,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAppInstallationState __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAppInstallationState;

typedef enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAvailability __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAvailability;

typedef enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentInteraction __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentInteraction;

typedef enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentObjectKind __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentObjectKind;

typedef enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentValueKind __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentValueKind;

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentAppInstallationState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAppInstallationState
{
    TargetedContentAppInstallationState_NotApplicable = 0,
    TargetedContentAppInstallationState_NotInstalled = 1,
    TargetedContentAppInstallationState_Installed = 2,
};
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentAvailability
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAvailability
{
    TargetedContentAvailability_None = 0,
    TargetedContentAvailability_Partial = 1,
    TargetedContentAvailability_All = 2,
};
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentInteraction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentInteraction
{
    TargetedContentInteraction_Impression = 0,
    TargetedContentInteraction_ClickThrough = 1,
    TargetedContentInteraction_Hover = 2,
    TargetedContentInteraction_Like = 3,
    TargetedContentInteraction_Dislike = 4,
    TargetedContentInteraction_Dismiss = 5,
    TargetedContentInteraction_Ineligible = 6,
    TargetedContentInteraction_Accept = 7,
    TargetedContentInteraction_Decline = 8,
    TargetedContentInteraction_Defer = 9,
    TargetedContentInteraction_Canceled = 10,
    TargetedContentInteraction_Conversion = 11,
    TargetedContentInteraction_Opportunity = 12,
};
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentObjectKind
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentObjectKind
{
    TargetedContentObjectKind_Collection = 0,
    TargetedContentObjectKind_Item = 1,
    TargetedContentObjectKind_Value = 2,
};
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Services.TargetedContent.TargetedContentValueKind
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentValueKind
{
    TargetedContentValueKind_String = 0,
    TargetedContentValueKind_Uri = 1,
    TargetedContentValueKind_Number = 2,
    TargetedContentValueKind_Boolean = 3,
    TargetedContentValueKind_File = 4,
    TargetedContentValueKind_ImageFile = 5,
    TargetedContentValueKind_Action = 6,
    TargetedContentValueKind_Strings = 7,
    TargetedContentValueKind_Uris = 8,
    TargetedContentValueKind_Numbers = 9,
    TargetedContentValueKind_Booleans = 10,
    TargetedContentValueKind_Files = 11,
    TargetedContentValueKind_ImageFiles = 12,
    TargetedContentValueKind_Actions = 13,
};
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentAction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentAction
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentAction[] = L"Windows.Services.TargetedContent.ITargetedContentAction";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InvokeAsync)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentActionVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_InvokeAsync(This, asyncAction) \
    ((This)->lpVtbl->InvokeAsync(This, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentAvailabilityChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentChangedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);
    HRESULT (STDMETHODCALLTYPE* get_HasPreviousContentExpired)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_get_HasPreviousContentExpired(This, value) \
    ((This)->lpVtbl->get_HasPreviousContentExpired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentCollection
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentCollection
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentCollection[] = L"Windows.Services.TargetedContent.ITargetedContentCollection";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportInteraction)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentInteraction interaction);
    HRESULT (STDMETHODCALLTYPE* ReportCustomInteraction)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        HSTRING customInteractionName);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** value);
    HRESULT (STDMETHODCALLTYPE* get_Collections)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection** value);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollectionVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_ReportInteraction(This, interaction) \
    ((This)->lpVtbl->ReportInteraction(This, interaction))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_ReportCustomInteraction(This, customInteractionName) \
    ((This)->lpVtbl->ReportCustomInteraction(This, customInteractionName))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_get_Collections(This, value) \
    ((This)->lpVtbl->get_Collections(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentContainer
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentContainer
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentContainer[] = L"Windows.Services.TargetedContent.ITargetedContentContainer";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Availability)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAvailability* value);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** value);
    HRESULT (STDMETHODCALLTYPE* SelectSingleObject)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer* This,
        HSTRING path,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_get_Availability(This, value) \
    ((This)->lpVtbl->get_Availability(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_SelectSingleObject(This, path, value) \
    ((This)->lpVtbl->SelectSingleObject(This, path, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentContainerStatics
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentContainer
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentContainerStatics[] = L"Windows.Services.TargetedContent.ITargetedContentContainerStatics";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsync)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics* This,
        HSTRING contentId,
        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer** asyncOperation);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStaticsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_GetAsync(This, contentId, asyncOperation) \
    ((This)->lpVtbl->GetAsync(This, contentId, asyncOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentContainerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentImage
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentImage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Storage.Streams.IRandomAccessStreamReference
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentImage[] = L"Windows.Services.TargetedContent.ITargetedContentImage";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImageVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentItem
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentItem
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentItem[] = L"Windows.Services.TargetedContent.ITargetedContentItem";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportInteraction)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentInteraction interaction);
    HRESULT (STDMETHODCALLTYPE* ReportCustomInteraction)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        HSTRING customInteractionName);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        __FIMapView_2_HSTRING_Windows__CServices__CTargetedContent__CTargetedContentValue** value);
    HRESULT (STDMETHODCALLTYPE* get_Collections)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentCollection** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_ReportInteraction(This, interaction) \
    ((This)->lpVtbl->ReportInteraction(This, interaction))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_ReportCustomInteraction(This, customInteractionName) \
    ((This)->lpVtbl->ReportCustomInteraction(This, customInteractionName))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_get_Collections(This, value) \
    ((This)->lpVtbl->get_Collections(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentItemState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentItemState
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentItemState[] = L"Windows.Services.TargetedContent.ITargetedContentItemState";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShouldDisplay)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AppInstallationState)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentAppInstallationState* value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemStateVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_get_ShouldDisplay(This, value) \
    ((This)->lpVtbl->get_ShouldDisplay(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_get_AppInstallationState(This, value) \
    ((This)->lpVtbl->get_AppInstallationState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItemState_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentObject
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentObject
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentObject[] = L"Windows.Services.TargetedContent.ITargetedContentObject";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ObjectKind)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentObjectKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Collection)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentCollection** value);
    HRESULT (STDMETHODCALLTYPE* get_Item)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentItem** value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObjectVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_get_ObjectKind(This, value) \
    ((This)->lpVtbl->get_ObjectKind(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_get_Collection(This, value) \
    ((This)->lpVtbl->get_Collection(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_get_Item(This, value) \
    ((This)->lpVtbl->get_Item(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentObject_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentStateChangedEventArgs[] = L"Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscription
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscription
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscription[] = L"Windows.Services.TargetedContent.ITargetedContentSubscription";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetContentContainerAsync)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentContainer** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* add_ContentChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ContentChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_AvailabilityChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentAvailabilityChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_AvailabilityChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        __FITypedEventHandler_2_Windows__CServices__CTargetedContent__CTargetedContentSubscription_Windows__CServices__CTargetedContent__CTargetedContentStateChangedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_GetContentContainerAsync(This, asyncOperation) \
    ((This)->lpVtbl->GetContentContainerAsync(This, asyncOperation))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_add_ContentChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_ContentChanged(This, handler, cookie))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_remove_ContentChanged(This, cookie) \
    ((This)->lpVtbl->remove_ContentChanged(This, cookie))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_add_AvailabilityChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_AvailabilityChanged(This, handler, cookie))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_remove_AvailabilityChanged(This, cookie) \
    ((This)->lpVtbl->remove_AvailabilityChanged(This, cookie))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_add_StateChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_StateChanged(This, handler, cookie))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_remove_StateChanged(This, cookie) \
    ((This)->lpVtbl->remove_StateChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscription_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscriptionOptions
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscriptionOptions[] = L"Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SubscriptionId)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AllowPartialContentAvailability)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowPartialContentAvailability)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CloudQueryParameters)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalFilters)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions* This);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptionsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_get_SubscriptionId(This, value) \
    ((This)->lpVtbl->get_SubscriptionId(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_get_AllowPartialContentAvailability(This, value) \
    ((This)->lpVtbl->get_AllowPartialContentAvailability(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_put_AllowPartialContentAvailability(This, value) \
    ((This)->lpVtbl->put_AllowPartialContentAvailability(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_get_CloudQueryParameters(This, value) \
    ((This)->lpVtbl->get_CloudQueryParameters(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_get_LocalFilters(This, value) \
    ((This)->lpVtbl->get_LocalFilters(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_Update(This) \
    ((This)->lpVtbl->Update(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentSubscription
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentSubscriptionStatics[] = L"Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAsync)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        HSTRING subscriptionId,
        __FIAsyncOperation_1_Windows__CServices__CTargetedContent__CTargetedContentSubscription** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* GetOptions)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics* This,
        HSTRING subscriptionId,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStaticsVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_GetAsync(This, subscriptionId, asyncOperation) \
    ((This)->lpVtbl->GetAsync(This, subscriptionId, asyncOperation))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_GetOptions(This, subscriptionId, value) \
    ((This)->lpVtbl->GetOptions(This, subscriptionId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentSubscriptionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Services.TargetedContent.ITargetedContentValue
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Services.TargetedContent.TargetedContentValue
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_TargetedContent_ITargetedContentValue[] = L"Windows.Services.TargetedContent.ITargetedContentValue";
typedef struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValueVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ValueKind)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        enum __x_ABI_CWindows_CServices_CTargetedContent_CTargetedContentValueKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_String)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Number)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Boolean)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageFile)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentImage** value);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentAction** value);
    HRESULT (STDMETHODCALLTYPE* get_Strings)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Uris)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_Numbers)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_double** value);
    HRESULT (STDMETHODCALLTYPE* get_Booleans)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* get_Files)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentFile** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageFiles)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentImage** value);
    HRESULT (STDMETHODCALLTYPE* get_Actions)(__x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue* This,
        __FIVectorView_1_Windows__CServices__CTargetedContent__CTargetedContentAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValueVtbl;

interface __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValueVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_ValueKind(This, value) \
    ((This)->lpVtbl->get_ValueKind(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_String(This, value) \
    ((This)->lpVtbl->get_String(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Number(This, value) \
    ((This)->lpVtbl->get_Number(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Boolean(This, value) \
    ((This)->lpVtbl->get_Boolean(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_ImageFile(This, value) \
    ((This)->lpVtbl->get_ImageFile(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Strings(This, value) \
    ((This)->lpVtbl->get_Strings(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Uris(This, value) \
    ((This)->lpVtbl->get_Uris(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Numbers(This, value) \
    ((This)->lpVtbl->get_Numbers(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Booleans(This, value) \
    ((This)->lpVtbl->get_Booleans(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Files(This, value) \
    ((This)->lpVtbl->get_Files(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_ImageFiles(This, value) \
    ((This)->lpVtbl->get_ImageFiles(This, value))

#define __x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_get_Actions(This, value) \
    ((This)->lpVtbl->get_Actions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue;
#endif /* !defined(____x_ABI_CWindows_CServices_CTargetedContent_CITargetedContentValue_INTERFACE_DEFINED__) */
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentAction
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentAction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAction_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentAction[] = L"Windows.Services.TargetedContent.TargetedContentAction";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentAvailabilityChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentCollection
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentCollection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentCollection_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentCollection[] = L"Windows.Services.TargetedContent.TargetedContentCollection";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentContainer
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.TargetedContent.ITargetedContentContainerStatics interface starting with version 1.0 of the Windows.Services.TargetedContent.TargetedContentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentContainer_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentContainer[] = L"Windows.Services.TargetedContent.TargetedContentContainer";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentFile
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Storage.Streams.IRandomAccessStreamReference ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentFile_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentFile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentFile[] = L"Windows.Services.TargetedContent.TargetedContentFile";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentImage
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentImage ** Default Interface **
 *    Windows.Storage.Streams.IRandomAccessStreamReference
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentImage_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentImage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentImage[] = L"Windows.Services.TargetedContent.TargetedContentImage";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentItem
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItem_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentItem[] = L"Windows.Services.TargetedContent.TargetedContentItem";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentItemState
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentItemState ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItemState_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentItemState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentItemState[] = L"Windows.Services.TargetedContent.TargetedContentItemState";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentObject
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentObject ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentObject_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentObject_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentObject[] = L"Windows.Services.TargetedContent.TargetedContentObject";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentStateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentStateChangedEventArgs[] = L"Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentSubscription
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.TargetedContent.ITargetedContentSubscriptionStatics interface starting with version 1.0 of the Windows.Services.TargetedContent.TargetedContentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentSubscription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscription_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentSubscription[] = L"Windows.Services.TargetedContent.TargetedContentSubscription";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentSubscriptionOptions
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentSubscriptionOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentSubscriptionOptions[] = L"Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Services.TargetedContent.TargetedContentValue
 *
 * Introduced to Windows.Services.TargetedContent.TargetedContentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Services.TargetedContent.ITargetedContentValue ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentValue_DEFINED
#define RUNTIMECLASS_Windows_Services_TargetedContent_TargetedContentValue_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_TargetedContent_TargetedContentValue[] = L"Windows.Services.TargetedContent.TargetedContentValue";
#endif
#endif // WINDOWS_SERVICES_TARGETEDCONTENT_TARGETEDCONTENTCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eservices2Etargetedcontent_p_h__

#endif // __windows2Eservices2Etargetedcontent_h__
