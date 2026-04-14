
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
#ifndef __windows2Emedia2Econtentrestrictions_h__
#define __windows2Emedia2Econtentrestrictions_h__
#ifndef __windows2Emedia2Econtentrestrictions_p_h__
#define __windows2Emedia2Econtentrestrictions_p_h__


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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                interface IContentRestrictionsBrowsePolicy;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy ABI::Windows::Media::ContentRestrictions::IContentRestrictionsBrowsePolicy

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                interface IRatedContentDescription;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription ABI::Windows::Media::ContentRestrictions::IRatedContentDescription

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                interface IRatedContentDescriptionFactory;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory ABI::Windows::Media::ContentRestrictions::IRatedContentDescriptionFactory

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                interface IRatedContentRestrictions;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions ABI::Windows::Media::ContentRestrictions::IRatedContentRestrictions

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                interface IRatedContentRestrictionsFactory;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory ABI::Windows::Media::ContentRestrictions::IRatedContentRestrictionsFactory

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__

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
        namespace Media {
            namespace ContentRestrictions {
                typedef enum ContentAccessRestrictionLevel : int ContentAccessRestrictionLevel;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("860c0179-be01-546d-a9ce-5956464c98ab"))
IAsyncOperation<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel> : IAsyncOperation_impl<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel> __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_t;
#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cf61be5d-40c3-5484-846a-3f82b8ba5738"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::ContentRestrictions::ContentAccessRestrictionLevel> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                class ContentRestrictionsBrowsePolicy;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ae3399b2-c7d5-5f1b-9fb9-f8bd81e9f9be"))
IAsyncOperation<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*, ABI::Windows::Media::ContentRestrictions::IContentRestrictionsBrowsePolicy*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*> __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_t;
#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("72ae1a16-c705-54e7-b1c4-fc05a0e07a77"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*, ABI::Windows::Media::ContentRestrictions::IContentRestrictionsBrowsePolicy*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::ContentRestrictions::ContentRestrictionsBrowsePolicy*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_USE */

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



#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */



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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                typedef enum RatedContentCategory : int RatedContentCategory;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                class RatedContentDescription;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                class RatedContentRestrictions;
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                enum ContentAccessRestrictionLevel : int
                {
                    ContentAccessRestrictionLevel_Allow = 0,
                    ContentAccessRestrictionLevel_Warn = 1,
                    ContentAccessRestrictionLevel_Block = 2,
                    ContentAccessRestrictionLevel_Hide = 3,
                };
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.ContentRestrictions.RatedContentCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                enum RatedContentCategory : int
                {
                    RatedContentCategory_General = 0,
                    RatedContentCategory_Application = 1,
                    RatedContentCategory_Game = 2,
                    RatedContentCategory_Movie = 3,
                    RatedContentCategory_Television = 4,
                    RatedContentCategory_Music = 5,
                };
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IContentRestrictionsBrowsePolicy[] = L"Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                MIDL_INTERFACE("8c0133a4-442e-461a-8757-fad2f5bd37e4")
                IContentRestrictionsBrowsePolicy : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_GeographicRegion(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxBrowsableAgeRating(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredAgeRating(
                        __FIReference_1_UINT32** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContentRestrictionsBrowsePolicy = __uuidof(IContentRestrictionsBrowsePolicy);
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentDescription[] = L"Windows.Media.ContentRestrictions.IRatedContentDescription";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                MIDL_INTERFACE("694866df-66b2-4dc3-96b1-f090eedee255")
                IRatedContentDescription : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Image(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Image(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Category(
                        ABI::Windows::Media::ContentRestrictions::RatedContentCategory* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Category(
                        ABI::Windows::Media::ContentRestrictions::RatedContentCategory value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ratings(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ratings(
                        __FIVector_1_HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRatedContentDescription = __uuidof(IRatedContentDescription);
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentDescriptionFactory[] = L"Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                MIDL_INTERFACE("2e38df62-9b90-4fa6-89c1-4b8d2ffb3573")
                IRatedContentDescriptionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING id,
                        HSTRING title,
                        ABI::Windows::Media::ContentRestrictions::RatedContentCategory category,
                        ABI::Windows::Media::ContentRestrictions::IRatedContentDescription** RatedContentDescription
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRatedContentDescriptionFactory = __uuidof(IRatedContentDescriptionFactory);
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentRestrictions[] = L"Windows.Media.ContentRestrictions.IRatedContentRestrictions";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                MIDL_INTERFACE("3f7f23cb-ba07-4401-a49d-8b9222205723")
                IRatedContentRestrictions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetBrowsePolicyAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRestrictionLevelAsync(
                        ABI::Windows::Media::ContentRestrictions::IRatedContentDescription* RatedContentDescription,
                        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestContentAccessAsync(
                        ABI::Windows::Media::ContentRestrictions::IRatedContentDescription* RatedContentDescription,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RestrictionsChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RestrictionsChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRatedContentRestrictions = __uuidof(IRatedContentRestrictions);
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentRestrictionsFactory[] = L"Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace ContentRestrictions {
                MIDL_INTERFACE("fb4b2996-c3bd-4910-9619-97cfd0694d56")
                IRatedContentRestrictionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithMaxAgeRating(
                        UINT32 maxAgeRating,
                        ABI::Windows::Media::ContentRestrictions::IRatedContentRestrictions** ratedContentRestrictions
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRatedContentRestrictionsFactory = __uuidof(IRatedContentRestrictionsFactory);
            } /* ContentRestrictions */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy[] = L"Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.RatedContentDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IRatedContentDescription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentDescription_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_RatedContentDescription[] = L"Windows.Media.ContentRestrictions.RatedContentDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IRatedContentRestrictions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentRestrictions_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentRestrictions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_RatedContentRestrictions[] = L"Windows.Media.ContentRestrictions.RatedContentRestrictions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy;

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription;

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory;

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions;

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory;

#endif // ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CMedia_CContentRestrictions_CContentAccessRestrictionLevel __x_ABI_CWindows_CMedia_CContentRestrictions_CContentAccessRestrictionLevel;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        enum __x_ABI_CWindows_CMedia_CContentRestrictions_CContentAccessRestrictionLevel* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* This,
        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* This,
        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__
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

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory;

/*
 *
 * Struct Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CContentRestrictions_CContentAccessRestrictionLevel
{
    ContentAccessRestrictionLevel_Allow = 0,
    ContentAccessRestrictionLevel_Warn = 1,
    ContentAccessRestrictionLevel_Block = 2,
    ContentAccessRestrictionLevel_Hide = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.ContentRestrictions.RatedContentCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory
{
    RatedContentCategory_General = 0,
    RatedContentCategory_Application = 1,
    RatedContentCategory_Game = 2,
    RatedContentCategory_Movie = 3,
    RatedContentCategory_Television = 4,
    RatedContentCategory_Music = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IContentRestrictionsBrowsePolicy[] = L"Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy";
typedef struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_GeographicRegion)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxBrowsableAgeRating)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredAgeRating)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy* This,
        __FIReference_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicyVtbl;

interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_get_GeographicRegion(This, value) \
    ((This)->lpVtbl->get_GeographicRegion(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_get_MaxBrowsableAgeRating(This, value) \
    ((This)->lpVtbl->get_MaxBrowsableAgeRating(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_get_PreferredAgeRating(This, value) \
    ((This)->lpVtbl->get_PreferredAgeRating(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIContentRestrictionsBrowsePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentDescription[] = L"Windows.Media.ContentRestrictions.IRatedContentDescription";
typedef struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Image)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_Image)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        enum __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory* value);
    HRESULT (STDMETHODCALLTYPE* put_Category)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        enum __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory value);
    HRESULT (STDMETHODCALLTYPE* get_Ratings)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_Ratings)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* This,
        __FIVector_1_HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionVtbl;

interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_get_Image(This, value) \
    ((This)->lpVtbl->get_Image(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_put_Image(This, value) \
    ((This)->lpVtbl->put_Image(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_put_Category(This, value) \
    ((This)->lpVtbl->put_Category(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_get_Ratings(This, value) \
    ((This)->lpVtbl->get_Ratings(This, value))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_put_Ratings(This, value) \
    ((This)->lpVtbl->put_Ratings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentDescription
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentDescriptionFactory[] = L"Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory";
typedef struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory* This,
        HSTRING id,
        HSTRING title,
        enum __x_ABI_CWindows_CMedia_CContentRestrictions_CRatedContentCategory category,
        __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription** RatedContentDescription);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_Create(This, id, title, category, RatedContentDescription) \
    ((This)->lpVtbl->Create(This, id, title, category, RatedContentDescription))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescriptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentRestrictions[] = L"Windows.Media.ContentRestrictions.IRatedContentRestrictions";
typedef struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetBrowsePolicyAsync)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentRestrictionsBrowsePolicy** operation);
    HRESULT (STDMETHODCALLTYPE* GetRestrictionLevelAsync)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* RatedContentDescription,
        __FIAsyncOperation_1_Windows__CMedia__CContentRestrictions__CContentAccessRestrictionLevel** operation);
    HRESULT (STDMETHODCALLTYPE* RequestContentAccessAsync)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentDescription* RatedContentDescription,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_RestrictionsChanged)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RestrictionsChanged)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsVtbl;

interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_GetBrowsePolicyAsync(This, operation) \
    ((This)->lpVtbl->GetBrowsePolicyAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_GetRestrictionLevelAsync(This, RatedContentDescription, operation) \
    ((This)->lpVtbl->GetRestrictionLevelAsync(This, RatedContentDescription, operation))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_RequestContentAccessAsync(This, RatedContentDescription, operation) \
    ((This)->lpVtbl->RequestContentAccessAsync(This, RatedContentDescription, operation))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_add_RestrictionsChanged(This, handler, token) \
    ((This)->lpVtbl->add_RestrictionsChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_remove_RestrictionsChanged(This, token) \
    ((This)->lpVtbl->remove_RestrictionsChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ContentRestrictions_IRatedContentRestrictionsFactory[] = L"Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory";
typedef struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithMaxAgeRating)(__x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory* This,
        UINT32 maxAgeRating,
        __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictions** ratedContentRestrictions);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_CreateWithMaxAgeRating(This, maxAgeRating, ratedContentRestrictions) \
    ((This)->lpVtbl->CreateWithMaxAgeRating(This, maxAgeRating, ratedContentRestrictions))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CContentRestrictions_CIRatedContentRestrictionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_ContentRestrictionsBrowsePolicy[] = L"Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.RatedContentDescription
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IRatedContentDescription ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentDescription_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentDescription_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_RatedContentDescription[] = L"Windows.Media.ContentRestrictions.RatedContentDescription";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ContentRestrictions.RatedContentRestrictions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ContentRestrictions.IRatedContentRestrictions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentRestrictions_DEFINED
#define RUNTIMECLASS_Windows_Media_ContentRestrictions_RatedContentRestrictions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ContentRestrictions_RatedContentRestrictions[] = L"Windows.Media.ContentRestrictions.RatedContentRestrictions";
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
#endif // __windows2Emedia2Econtentrestrictions_p_h__

#endif // __windows2Emedia2Econtentrestrictions_h__
