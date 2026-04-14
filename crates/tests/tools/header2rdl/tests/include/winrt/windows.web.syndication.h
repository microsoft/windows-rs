
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
#ifndef __windows2Eweb2Esyndication_h__
#define __windows2Eweb2Esyndication_h__
#ifndef __windows2Eweb2Esyndication_p_h__
#define __windows2Eweb2Esyndication_p_h__


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
#include "Windows.Data.Xml.Dom.h"
#include "Windows.Security.Credentials.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationAttribute;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute ABI::Windows::Web::Syndication::ISyndicationAttribute

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationAttributeFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory ABI::Windows::Web::Syndication::ISyndicationAttributeFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationCategory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory ABI::Windows::Web::Syndication::ISyndicationCategory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationCategoryFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory ABI::Windows::Web::Syndication::ISyndicationCategoryFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationClient;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient ABI::Windows::Web::Syndication::ISyndicationClient

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationClientFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory ABI::Windows::Web::Syndication::ISyndicationClientFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationContent;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent ABI::Windows::Web::Syndication::ISyndicationContent

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationContentFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory ABI::Windows::Web::Syndication::ISyndicationContentFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationErrorStatics;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics ABI::Windows::Web::Syndication::ISyndicationErrorStatics

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationFeed;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed ABI::Windows::Web::Syndication::ISyndicationFeed

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationFeedFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory ABI::Windows::Web::Syndication::ISyndicationFeedFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationGenerator;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator ABI::Windows::Web::Syndication::ISyndicationGenerator

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationGeneratorFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory ABI::Windows::Web::Syndication::ISyndicationGeneratorFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationItem;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem ABI::Windows::Web::Syndication::ISyndicationItem

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationItemFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory ABI::Windows::Web::Syndication::ISyndicationItemFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationLink;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink ABI::Windows::Web::Syndication::ISyndicationLink

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationLinkFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory ABI::Windows::Web::Syndication::ISyndicationLinkFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationNode;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode ABI::Windows::Web::Syndication::ISyndicationNode

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationNodeFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory ABI::Windows::Web::Syndication::ISyndicationNodeFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationPerson;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson ABI::Windows::Web::Syndication::ISyndicationPerson

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationPersonFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory ABI::Windows::Web::Syndication::ISyndicationPersonFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationText;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText ABI::Windows::Web::Syndication::ISyndicationText

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                interface ISyndicationTextFactory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory ABI::Windows::Web::Syndication::ISyndicationTextFactory

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationFeed;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                typedef struct RetrievalProgress RetrievalProgress;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0e3d7f70-4e8c-5260-a7e5-786e05bded99"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationFeed*, ABI::Windows::Web::Syndication::ISyndicationFeed*>, struct ABI::Windows::Web::Syndication::RetrievalProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Syndication.SyndicationFeed, Windows.Web.Syndication.RetrievalProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92eaf151-415e-5f87-8095-781623c88998"))
IAsyncOperationWithProgress<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationFeed*, ABI::Windows::Web::Syndication::ISyndicationFeed*>, struct ABI::Windows::Web::Syndication::RetrievalProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Syndication.SyndicationFeed, Windows.Web.Syndication.RetrievalProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1017bbe0-9d10-543e-8f03-885122a082f3"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationFeed*, ABI::Windows::Web::Syndication::ISyndicationFeed*>, struct ABI::Windows::Web::Syndication::RetrievalProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Syndication.SyndicationFeed, Windows.Web.Syndication.RetrievalProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Syndication::SyndicationFeed*, struct ABI::Windows::Web::Syndication::RetrievalProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2dc8d9d6-0f44-5692-933e-f8902ab7fb94"))
IIterator<ABI::Windows::Web::Syndication::ISyndicationNode*> : IIterator_impl<ABI::Windows::Web::Syndication::ISyndicationNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.ISyndicationNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::ISyndicationNode*> __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b486569a-72b3-57aa-9950-cea0b3e4fc58"))
IIterable<ABI::Windows::Web::Syndication::ISyndicationNode*> : IIterable_impl<ABI::Windows::Web::Syndication::ISyndicationNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.ISyndicationNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::ISyndicationNode*> __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationAttribute;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a76fcde8-f86f-5b75-aa7d-5787467a319d"))
IIterator<ABI::Windows::Web::Syndication::SyndicationAttribute*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationAttribute*, ABI::Windows::Web::Syndication::ISyndicationAttribute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.SyndicationAttribute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::SyndicationAttribute*> __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("329eabe1-efcc-539e-96ba-f6a44f221dbd"))
IIterable<ABI::Windows::Web::Syndication::SyndicationAttribute*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationAttribute*, ABI::Windows::Web::Syndication::ISyndicationAttribute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.SyndicationAttribute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::SyndicationAttribute*> __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationCategory;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2a9228fa-b088-5690-bb38-b7044e0b502b"))
IIterator<ABI::Windows::Web::Syndication::SyndicationCategory*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationCategory*, ABI::Windows::Web::Syndication::ISyndicationCategory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.SyndicationCategory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::SyndicationCategory*> __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d151f7d1-eabd-5300-b55c-149eb289cc71"))
IIterable<ABI::Windows::Web::Syndication::SyndicationCategory*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationCategory*, ABI::Windows::Web::Syndication::ISyndicationCategory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.SyndicationCategory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::SyndicationCategory*> __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationItem;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d5692aa3-d785-5db4-ac5c-b3832082e629"))
IIterator<ABI::Windows::Web::Syndication::SyndicationItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationItem*, ABI::Windows::Web::Syndication::ISyndicationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.SyndicationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::SyndicationItem*> __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55463eef-ecb8-59cd-8d6b-74daacbe7d19"))
IIterable<ABI::Windows::Web::Syndication::SyndicationItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationItem*, ABI::Windows::Web::Syndication::ISyndicationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.SyndicationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::SyndicationItem*> __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationLink;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("901642b7-6ca4-5b57-b8f1-73208342ba4a"))
IIterator<ABI::Windows::Web::Syndication::SyndicationLink*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationLink*, ABI::Windows::Web::Syndication::ISyndicationLink*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.SyndicationLink>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::SyndicationLink*> __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c6919f6a-66d9-556a-9632-87d39af14638"))
IIterable<ABI::Windows::Web::Syndication::SyndicationLink*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationLink*, ABI::Windows::Web::Syndication::ISyndicationLink*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.SyndicationLink>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::SyndicationLink*> __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationPerson;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#define DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1745e807-f209-5da6-8855-7f99e25eb1fc"))
IIterator<ABI::Windows::Web::Syndication::SyndicationPerson*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationPerson*, ABI::Windows::Web::Syndication::ISyndicationPerson*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Syndication.SyndicationPerson>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Syndication::SyndicationPerson*> __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_t;
#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#define DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e58e7844-eb34-5284-b09e-de6762d548ca"))
IIterable<ABI::Windows::Web::Syndication::SyndicationPerson*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationPerson*, ABI::Windows::Web::Syndication::ISyndicationPerson*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Syndication.SyndicationPerson>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Syndication::SyndicationPerson*> __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_t;
#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6b4bd4a1-b4f6-5433-afd7-bd2e501a1041"))
IVectorView<ABI::Windows::Web::Syndication::ISyndicationNode*> : IVectorView_impl<ABI::Windows::Web::Syndication::ISyndicationNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.ISyndicationNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::ISyndicationNode*> __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c1d6d1cc-69ce-5486-9f35-c87e13111387"))
IVectorView<ABI::Windows::Web::Syndication::SyndicationAttribute*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationAttribute*, ABI::Windows::Web::Syndication::ISyndicationAttribute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.SyndicationAttribute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::SyndicationAttribute*> __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a1ac007c-9d94-552e-840e-139f109a9b88"))
IVectorView<ABI::Windows::Web::Syndication::SyndicationCategory*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationCategory*, ABI::Windows::Web::Syndication::ISyndicationCategory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.SyndicationCategory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::SyndicationCategory*> __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9496279b-567e-5652-b942-f6fb70c34173"))
IVectorView<ABI::Windows::Web::Syndication::SyndicationItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationItem*, ABI::Windows::Web::Syndication::ISyndicationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.SyndicationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::SyndicationItem*> __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb8b7ff6-fa64-576a-8be4-a055f7a04a73"))
IVectorView<ABI::Windows::Web::Syndication::SyndicationLink*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationLink*, ABI::Windows::Web::Syndication::ISyndicationLink*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.SyndicationLink>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::SyndicationLink*> __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#define DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0e450d3d-e750-5787-885b-488abc72b5b9"))
IVectorView<ABI::Windows::Web::Syndication::SyndicationPerson*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationPerson*, ABI::Windows::Web::Syndication::ISyndicationPerson*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Syndication.SyndicationPerson>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Syndication::SyndicationPerson*> __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_t;
#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4508afa-9f02-5eb8-a389-14bbe5193ac0"))
IVector<ABI::Windows::Web::Syndication::ISyndicationNode*> : IVector_impl<ABI::Windows::Web::Syndication::ISyndicationNode*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.ISyndicationNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::ISyndicationNode*> __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2fe84e7b-2350-5941-94b5-a64677b585d1"))
IVector<ABI::Windows::Web::Syndication::SyndicationAttribute*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationAttribute*, ABI::Windows::Web::Syndication::ISyndicationAttribute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.SyndicationAttribute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::SyndicationAttribute*> __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("72e456e4-0e52-52cb-b363-f3581327f033"))
IVector<ABI::Windows::Web::Syndication::SyndicationCategory*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationCategory*, ABI::Windows::Web::Syndication::ISyndicationCategory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.SyndicationCategory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::SyndicationCategory*> __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aa01130b-4631-5117-8c48-dc21b0295096"))
IVector<ABI::Windows::Web::Syndication::SyndicationItem*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationItem*, ABI::Windows::Web::Syndication::ISyndicationItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.SyndicationItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::SyndicationItem*> __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b8fb25a5-01c3-5207-814e-892b2b5343f7"))
IVector<ABI::Windows::Web::Syndication::SyndicationLink*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationLink*, ABI::Windows::Web::Syndication::ISyndicationLink*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.SyndicationLink>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::SyndicationLink*> __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#define DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ab772cd6-8ce7-5db9-83ac-0db9e44a1b0c"))
IVector<ABI::Windows::Web::Syndication::SyndicationPerson*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Syndication::SyndicationPerson*, ABI::Windows::Web::Syndication::ISyndicationPerson*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Syndication.SyndicationPerson>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Syndication::SyndicationPerson*> __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_t;
#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument ABI::Windows::Data::Xml::Dom::IXmlDocument

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

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
        namespace Security {
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IPasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential ABI::Windows::Security::Credentials::IPasswordCredential

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                typedef enum SyndicationErrorStatus : int SyndicationErrorStatus;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                typedef enum SyndicationFormat : int SyndicationFormat;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                typedef enum SyndicationTextType : int SyndicationTextType;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationClient;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationContent;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationGenerator;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationNode;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                class SyndicationText;
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.Syndication.SyndicationErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                enum SyndicationErrorStatus : int
                {
                    SyndicationErrorStatus_Unknown = 0,
                    SyndicationErrorStatus_MissingRequiredElement = 1,
                    SyndicationErrorStatus_MissingRequiredAttribute = 2,
                    SyndicationErrorStatus_InvalidXml = 3,
                    SyndicationErrorStatus_UnexpectedContent = 4,
                    SyndicationErrorStatus_UnsupportedFormat = 5,
                };
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.SyndicationFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                enum SyndicationFormat : int
                {
                    SyndicationFormat_Atom10 = 0,
                    SyndicationFormat_Rss20 = 1,
                    SyndicationFormat_Rss10 = 2,
                    SyndicationFormat_Rss092 = 3,
                    SyndicationFormat_Rss091 = 4,
                    SyndicationFormat_Atom03 = 5,
                };
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.SyndicationTextType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                enum SyndicationTextType : int
                {
                    SyndicationTextType_Text = 0,
                    SyndicationTextType_Html = 1,
                    SyndicationTextType_Xhtml = 2,
                };
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.RetrievalProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                struct RetrievalProgress
                {
                    UINT32 BytesRetrieved;
                    UINT32 TotalBytesToRetrieve;
                };
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.TransferProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                struct TransferProgress
                {
                    UINT32 BytesSent;
                    UINT32 TotalBytesToSend;
                    UINT32 BytesRetrieved;
                    UINT32 TotalBytesToRetrieve;
                };
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationAttribute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationAttribute[] = L"Windows.Web.Syndication.ISyndicationAttribute";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("71e8f969-526e-4001-9a91-e84f83161ab1")
                ISyndicationAttribute : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Namespace(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Namespace(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationAttribute = __uuidof(ISyndicationAttribute);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationAttributeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationAttribute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationAttributeFactory[] = L"Windows.Web.Syndication.ISyndicationAttributeFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("624f1599-ed3e-420f-be86-640414886e4b")
                ISyndicationAttributeFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationAttribute(
                        HSTRING attributeName,
                        HSTRING attributeNamespace,
                        HSTRING attributeValue,
                        ABI::Windows::Web::Syndication::ISyndicationAttribute** syndicationAttribute
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationAttributeFactory = __uuidof(ISyndicationAttributeFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationCategory
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationCategory[] = L"Windows.Web.Syndication.ISyndicationCategory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("8715626f-0cba-4a7f-89ff-ecb5281423b6")
                ISyndicationCategory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Label(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Scheme(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Scheme(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Term(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Term(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationCategory = __uuidof(ISyndicationCategory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationCategoryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationCategory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationCategoryFactory[] = L"Windows.Web.Syndication.ISyndicationCategoryFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("ab42802f-49e0-4525-8ab2-ab45c02528ff")
                ISyndicationCategoryFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationCategory(
                        HSTRING term,
                        ABI::Windows::Web::Syndication::ISyndicationCategory** category
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationCategoryEx(
                        HSTRING term,
                        HSTRING scheme,
                        HSTRING label,
                        ABI::Windows::Web::Syndication::ISyndicationCategory** category
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationCategoryFactory = __uuidof(ISyndicationCategoryFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationClient[] = L"Windows.Web.Syndication.ISyndicationClient";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("9e18a9b7-7249-4b45-b229-7df895a5a1f5")
                ISyndicationClient : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxResponseBufferSize(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxResponseBufferSize(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timeout(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Timeout(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BypassCacheOnRetrieve(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BypassCacheOnRetrieve(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetRequestHeader(
                        HSTRING name,
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrieveFeedAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationClient = __uuidof(ISyndicationClient);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationClientFactory[] = L"Windows.Web.Syndication.ISyndicationClientFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("2ec4b32c-a79b-4114-b29a-05dffbafb9a4")
                ISyndicationClientFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationClient(
                        ABI::Windows::Security::Credentials::IPasswordCredential* serverCredential,
                        ABI::Windows::Web::Syndication::ISyndicationClient** syndicationClient
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationClientFactory = __uuidof(ISyndicationClientFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationContent
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationText
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationContent[] = L"Windows.Web.Syndication.ISyndicationContent";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("4641fefe-0e55-40d0-b8d0-6a2ccba9fc7c")
                ISyndicationContent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationContent = __uuidof(ISyndicationContent);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationContentFactory[] = L"Windows.Web.Syndication.ISyndicationContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("3d2fbb93-9520-4173-9388-7e2df324a8a0")
                ISyndicationContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationContent(
                        HSTRING text,
                        ABI::Windows::Web::Syndication::SyndicationTextType type,
                        ABI::Windows::Web::Syndication::ISyndicationContent** content
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationContentWithSourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* sourceUri,
                        ABI::Windows::Web::Syndication::ISyndicationContent** content
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationContentFactory = __uuidof(ISyndicationContentFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationErrorStatics[] = L"Windows.Web.Syndication.ISyndicationErrorStatics";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("1fbb2361-45c7-4833-8aa0-be5f3b58a7f4")
                ISyndicationErrorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        INT32 hresult,
                        ABI::Windows::Web::Syndication::SyndicationErrorStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationErrorStatics = __uuidof(ISyndicationErrorStatics);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationFeed
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationFeed
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationFeed[] = L"Windows.Web.Syndication.ISyndicationFeed";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("7ffe3cd2-5b66-4d62-8403-1bc10d910d6b")
                ISyndicationFeed : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Authors(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Categories(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contributors(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Generator(
                        ABI::Windows::Web::Syndication::ISyndicationGenerator** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Generator(
                        ABI::Windows::Web::Syndication::ISyndicationGenerator* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IconUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IconUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Items(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdatedTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastUpdatedTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Links(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImageUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ImageUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rights(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rights(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subtitle(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FirstUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NextUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreviousUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceFormat(
                        ABI::Windows::Web::Syndication::SyndicationFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Load(
                        HSTRING feed
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromXml(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* feedDocument
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationFeed = __uuidof(ISyndicationFeed);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationFeedFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationFeed
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationFeedFactory[] = L"Windows.Web.Syndication.ISyndicationFeedFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("23472232-8be9-48b7-8934-6205131d9357")
                ISyndicationFeedFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationFeed(
                        HSTRING title,
                        HSTRING subtitle,
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Syndication::ISyndicationFeed** feed
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationFeedFactory = __uuidof(ISyndicationFeedFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationGenerator[] = L"Windows.Web.Syndication.ISyndicationGenerator";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("9768b379-fb2b-4f6d-b41c-088a5868825c")
                ISyndicationGenerator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Version(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationGenerator = __uuidof(ISyndicationGenerator);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationGeneratorFactory[] = L"Windows.Web.Syndication.ISyndicationGeneratorFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("a34083e3-1e26-4dbc-ba9d-1ab84beff97b")
                ISyndicationGeneratorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationGenerator(
                        HSTRING text,
                        ABI::Windows::Web::Syndication::ISyndicationGenerator** generator
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationGeneratorFactory = __uuidof(ISyndicationGeneratorFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationItem
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationItem[] = L"Windows.Web.Syndication.ISyndicationItem";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("548db883-c384-45c1-8ae8-a378c4ec486c")
                ISyndicationItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Authors(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Categories(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contributors(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Web::Syndication::ISyndicationContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Content(
                        ABI::Windows::Web::Syndication::ISyndicationContent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdatedTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastUpdatedTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Links(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PublishedDate(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PublishedDate(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rights(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rights(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Web::Syndication::ISyndicationFeed** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Source(
                        ABI::Windows::Web::Syndication::ISyndicationFeed* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Summary(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Summary(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        ABI::Windows::Web::Syndication::ISyndicationText** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        ABI::Windows::Web::Syndication::ISyndicationText* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CommentsUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CommentsUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EditUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EditMediaUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ETag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ItemUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Load(
                        HSTRING item
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LoadFromXml(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* itemDocument
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationItem = __uuidof(ISyndicationItem);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationItemFactory[] = L"Windows.Web.Syndication.ISyndicationItemFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("251d434f-7db8-487a-85e4-10d191e66ebb")
                ISyndicationItemFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationItem(
                        HSTRING title,
                        ABI::Windows::Web::Syndication::ISyndicationContent* content,
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Syndication::ISyndicationItem** item
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationItemFactory = __uuidof(ISyndicationItemFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationLink
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationLink[] = L"Windows.Web.Syndication.ISyndicationLink";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("27553abd-a10e-41b5-86bd-9759086eb0c5")
                ISyndicationLink : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Length(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Length(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MediaType(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Relationship(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Relationship(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResourceLanguage(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ResourceLanguage(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationLink = __uuidof(ISyndicationLink);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationLinkFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationLinkFactory[] = L"Windows.Web.Syndication.ISyndicationLinkFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("5ed863d4-5535-48ac-98d4-c190995080b3")
                ISyndicationLinkFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Syndication::ISyndicationLink** link
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationLinkEx(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        HSTRING relationship,
                        HSTRING title,
                        HSTRING mediaType,
                        UINT32 length,
                        ABI::Windows::Web::Syndication::ISyndicationLink** link
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationLinkFactory = __uuidof(ISyndicationLinkFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationNode[] = L"Windows.Web.Syndication.ISyndicationNode";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("753cef78-51f8-45c0-a9f5-f1719dec3fb2")
                ISyndicationNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NodeName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NodeName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NodeNamespace(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NodeNamespace(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NodeValue(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NodeValue(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Language(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BaseUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BaseUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttributeExtensions(
                        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ElementExtensions(
                        __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetXmlDocument(
                        ABI::Windows::Web::Syndication::SyndicationFormat format,
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** xmlDocument
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationNode = __uuidof(ISyndicationNode);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationNodeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationNodeFactory[] = L"Windows.Web.Syndication.ISyndicationNodeFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("12902188-4acb-49a8-b777-a5eb92e18a79")
                ISyndicationNodeFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationNode(
                        HSTRING nodeName,
                        HSTRING nodeNamespace,
                        HSTRING nodeValue,
                        ABI::Windows::Web::Syndication::ISyndicationNode** node
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationNodeFactory = __uuidof(ISyndicationNodeFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationPerson
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationPerson
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationPerson[] = L"Windows.Web.Syndication.ISyndicationPerson";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("fa1ee5da-a7c6-4517-a096-0143faf29327")
                ISyndicationPerson : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Email(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Email(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationPerson = __uuidof(ISyndicationPerson);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationPersonFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationPerson
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationPersonFactory[] = L"Windows.Web.Syndication.ISyndicationPersonFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("dcf4886d-229d-4b58-a49b-f3d2f0f5c99f")
                ISyndicationPersonFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationPerson(
                        HSTRING name,
                        ABI::Windows::Web::Syndication::ISyndicationPerson** person
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationPersonEx(
                        HSTRING name,
                        HSTRING email,
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Syndication::ISyndicationPerson** person
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationPersonFactory = __uuidof(ISyndicationPersonFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationText[] = L"Windows.Web.Syndication.ISyndicationText";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("b9cc5e80-313a-4091-a2a6-243e0ee923f9")
                ISyndicationText : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Type(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Xml(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Xml(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationText = __uuidof(ISyndicationText);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationText;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationTextFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationText
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationTextFactory[] = L"Windows.Web.Syndication.ISyndicationTextFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Syndication {
                MIDL_INTERFACE("ee7342f7-11c6-4b25-ab62-e596bd162946")
                ISyndicationTextFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationText(
                        HSTRING text,
                        ABI::Windows::Web::Syndication::ISyndicationText** syndicationText
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSyndicationTextEx(
                        HSTRING text,
                        ABI::Windows::Web::Syndication::SyndicationTextType type,
                        ABI::Windows::Web::Syndication::ISyndicationText** syndicationText
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISyndicationTextFactory = __uuidof(ISyndicationTextFactory);
            } /* Syndication */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationAttributeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationAttribute ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationAttribute_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationAttribute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationAttribute[] = L"Windows.Web.Syndication.SyndicationAttribute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationCategoryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationCategory ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationCategory_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationCategory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationCategory[] = L"Windows.Web.Syndication.SyndicationCategory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationClient ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationClient_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationClient[] = L"Windows.Web.Syndication.SyndicationClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationText
 *    Windows.Web.Syndication.ISyndicationNode
 *    Windows.Web.Syndication.ISyndicationContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationContent[] = L"Windows.Web.Syndication.SyndicationContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Syndication.ISyndicationErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationError_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationError[] = L"Windows.Web.Syndication.SyndicationError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationFeed
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationFeedFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationFeed ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationFeed_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationFeed_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationFeed[] = L"Windows.Web.Syndication.SyndicationFeed";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationGenerator ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationGenerator_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationGenerator[] = L"Windows.Web.Syndication.SyndicationGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationItem ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationItem_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationItem[] = L"Windows.Web.Syndication.SyndicationItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationLinkFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationLink ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationLink_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationLink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationLink[] = L"Windows.Web.Syndication.SyndicationLink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationNodeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationNode_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationNode[] = L"Windows.Web.Syndication.SyndicationNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationPerson
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationPersonFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationPerson ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationPerson_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationPerson_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationPerson[] = L"Windows.Web.Syndication.SyndicationPerson";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationTextFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationText ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationText_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationText[] = L"Windows.Web.Syndication.SyndicationText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory;

#endif // ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CWeb_CSyndication_CRetrievalProgress __x_ABI_CWindows_CWeb_CSyndication_CRetrievalProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CSyndication_CRetrievalProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CISyndicationNode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationAttribute** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationCategory** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationLink** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson;

typedef struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl;

interface __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson;

typedef struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __FIIterator_1_Windows__CWeb__CSyndication__CSyndicationPerson** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl;

interface __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson;

typedef struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl;

interface __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CISyndicationNode** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationAttribute** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationCategory** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationItem** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationLink** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson;

typedef struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __FIVectorView_1_Windows__CWeb__CSyndication__CSyndicationPerson** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl;

interface __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPersonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationErrorStatus __x_ABI_CWindows_CWeb_CSyndication_CSyndicationErrorStatus;

typedef enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationFormat __x_ABI_CWindows_CWeb_CSyndication_CSyndicationFormat;

typedef enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationTextType __x_ABI_CWindows_CWeb_CSyndication_CSyndicationTextType;

/*
 *
 * Struct Windows.Web.Syndication.SyndicationErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationErrorStatus
{
    SyndicationErrorStatus_Unknown = 0,
    SyndicationErrorStatus_MissingRequiredElement = 1,
    SyndicationErrorStatus_MissingRequiredAttribute = 2,
    SyndicationErrorStatus_InvalidXml = 3,
    SyndicationErrorStatus_UnexpectedContent = 4,
    SyndicationErrorStatus_UnsupportedFormat = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.SyndicationFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationFormat
{
    SyndicationFormat_Atom10 = 0,
    SyndicationFormat_Rss20 = 1,
    SyndicationFormat_Rss10 = 2,
    SyndicationFormat_Rss092 = 3,
    SyndicationFormat_Rss091 = 4,
    SyndicationFormat_Atom03 = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.SyndicationTextType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationTextType
{
    SyndicationTextType_Text = 0,
    SyndicationTextType_Html = 1,
    SyndicationTextType_Xhtml = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.RetrievalProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CWeb_CSyndication_CRetrievalProgress
{
    UINT32 BytesRetrieved;
    UINT32 TotalBytesToRetrieve;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Syndication.TransferProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CWeb_CSyndication_CTransferProgress
{
    UINT32 BytesSent;
    UINT32 TotalBytesToSend;
    UINT32 BytesRetrieved;
    UINT32 TotalBytesToRetrieve;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationAttribute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationAttribute[] = L"Windows.Web.Syndication.ISyndicationAttribute";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Namespace)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Namespace)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_get_Namespace(This, value) \
    ((This)->lpVtbl->get_Namespace(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_put_Namespace(This, value) \
    ((This)->lpVtbl->put_Namespace(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationAttributeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationAttribute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationAttributeFactory[] = L"Windows.Web.Syndication.ISyndicationAttributeFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationAttribute)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory* This,
        HSTRING attributeName,
        HSTRING attributeNamespace,
        HSTRING attributeValue,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttribute** syndicationAttribute);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_CreateSyndicationAttribute(This, attributeName, attributeNamespace, attributeValue, syndicationAttribute) \
    ((This)->lpVtbl->CreateSyndicationAttribute(This, attributeName, attributeNamespace, attributeValue, syndicationAttribute))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationAttributeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationCategory
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationCategory[] = L"Windows.Web.Syndication.ISyndicationCategory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Label)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Scheme)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Scheme)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Term)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Term)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_put_Label(This, value) \
    ((This)->lpVtbl->put_Label(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_get_Scheme(This, value) \
    ((This)->lpVtbl->get_Scheme(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_put_Scheme(This, value) \
    ((This)->lpVtbl->put_Scheme(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_get_Term(This, value) \
    ((This)->lpVtbl->get_Term(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_put_Term(This, value) \
    ((This)->lpVtbl->put_Term(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationCategoryFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationCategory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationCategoryFactory[] = L"Windows.Web.Syndication.ISyndicationCategoryFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationCategory)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        HSTRING term,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** category);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationCategoryEx)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory* This,
        HSTRING term,
        HSTRING scheme,
        HSTRING label,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategory** category);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_CreateSyndicationCategory(This, term, category) \
    ((This)->lpVtbl->CreateSyndicationCategory(This, term, category))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_CreateSyndicationCategoryEx(This, term, scheme, label, category) \
    ((This)->lpVtbl->CreateSyndicationCategoryEx(This, term, scheme, label, category))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationCategoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationClient[] = L"Windows.Web.Syndication.ISyndicationClient";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerCredential)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ServerCredential)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_ProxyCredential)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ProxyCredential)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxResponseBufferSize)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxResponseBufferSize)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Timeout)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Timeout)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BypassCacheOnRetrieve)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_BypassCacheOnRetrieve)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetRequestHeader)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        HSTRING name,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RetrieveFeedAsync)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CSyndication__CSyndicationFeed_Windows__CWeb__CSyndication__CRetrievalProgress** operation);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_get_ServerCredential(This, value) \
    ((This)->lpVtbl->get_ServerCredential(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_put_ServerCredential(This, value) \
    ((This)->lpVtbl->put_ServerCredential(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_get_ProxyCredential(This, value) \
    ((This)->lpVtbl->get_ProxyCredential(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_put_ProxyCredential(This, value) \
    ((This)->lpVtbl->put_ProxyCredential(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_get_MaxResponseBufferSize(This, value) \
    ((This)->lpVtbl->get_MaxResponseBufferSize(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_put_MaxResponseBufferSize(This, value) \
    ((This)->lpVtbl->put_MaxResponseBufferSize(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_get_Timeout(This, value) \
    ((This)->lpVtbl->get_Timeout(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_put_Timeout(This, value) \
    ((This)->lpVtbl->put_Timeout(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_get_BypassCacheOnRetrieve(This, value) \
    ((This)->lpVtbl->get_BypassCacheOnRetrieve(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_put_BypassCacheOnRetrieve(This, value) \
    ((This)->lpVtbl->put_BypassCacheOnRetrieve(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_SetRequestHeader(This, name, value) \
    ((This)->lpVtbl->SetRequestHeader(This, name, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_RetrieveFeedAsync(This, uri, operation) \
    ((This)->lpVtbl->RetrieveFeedAsync(This, uri, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationClientFactory[] = L"Windows.Web.Syndication.ISyndicationClientFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationClient)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* serverCredential,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClient** syndicationClient);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_CreateSyndicationClient(This, serverCredential, syndicationClient) \
    ((This)->lpVtbl->CreateSyndicationClient(This, serverCredential, syndicationClient))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationContent
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationText
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationContent[] = L"Windows.Web.Syndication.ISyndicationContent";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_SourceUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_get_SourceUri(This, value) \
    ((This)->lpVtbl->get_SourceUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_put_SourceUri(This, value) \
    ((This)->lpVtbl->put_SourceUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationContentFactory[] = L"Windows.Web.Syndication.ISyndicationContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationContent)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        HSTRING text,
        enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationTextType type,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent** content);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationContentWithSourceUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* sourceUri,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent** content);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_CreateSyndicationContent(This, text, type, content) \
    ((This)->lpVtbl->CreateSyndicationContent(This, text, type, content))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_CreateSyndicationContentWithSourceUri(This, sourceUri, content) \
    ((This)->lpVtbl->CreateSyndicationContentWithSourceUri(This, sourceUri, content))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationErrorStatics[] = L"Windows.Web.Syndication.ISyndicationErrorStatics";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics* This,
        INT32 hresult,
        enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationErrorStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStaticsVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_GetStatus(This, hresult, status) \
    ((This)->lpVtbl->GetStatus(This, hresult, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationFeed
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationFeed
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationFeed[] = L"Windows.Web.Syndication.ISyndicationFeed";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Authors)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value);
    HRESULT (STDMETHODCALLTYPE* get_Categories)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory** value);
    HRESULT (STDMETHODCALLTYPE* get_Contributors)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value);
    HRESULT (STDMETHODCALLTYPE* get_Generator)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator** value);
    HRESULT (STDMETHODCALLTYPE* put_Generator)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* value);
    HRESULT (STDMETHODCALLTYPE* get_IconUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_IconUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationItem** value);
    HRESULT (STDMETHODCALLTYPE* get_LastUpdatedTime)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_LastUpdatedTime)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Links)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ImageUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Rights)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Rights)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Subtitle)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_FirstUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_LastUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_NextUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_PreviousUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_SourceFormat)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationFormat* value);
    HRESULT (STDMETHODCALLTYPE* Load)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        HSTRING feed);
    HRESULT (STDMETHODCALLTYPE* LoadFromXml)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* feedDocument);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Authors(This, value) \
    ((This)->lpVtbl->get_Authors(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Categories(This, value) \
    ((This)->lpVtbl->get_Categories(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Contributors(This, value) \
    ((This)->lpVtbl->get_Contributors(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Generator(This, value) \
    ((This)->lpVtbl->get_Generator(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_Generator(This, value) \
    ((This)->lpVtbl->put_Generator(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_IconUri(This, value) \
    ((This)->lpVtbl->get_IconUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_IconUri(This, value) \
    ((This)->lpVtbl->put_IconUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->get_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->put_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Links(This, value) \
    ((This)->lpVtbl->get_Links(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_ImageUri(This, value) \
    ((This)->lpVtbl->get_ImageUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_ImageUri(This, value) \
    ((This)->lpVtbl->put_ImageUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Rights(This, value) \
    ((This)->lpVtbl->get_Rights(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_Rights(This, value) \
    ((This)->lpVtbl->put_Rights(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_Subtitle(This, value) \
    ((This)->lpVtbl->put_Subtitle(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_FirstUri(This, value) \
    ((This)->lpVtbl->get_FirstUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_LastUri(This, value) \
    ((This)->lpVtbl->get_LastUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_NextUri(This, value) \
    ((This)->lpVtbl->get_NextUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_PreviousUri(This, value) \
    ((This)->lpVtbl->get_PreviousUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_get_SourceFormat(This, value) \
    ((This)->lpVtbl->get_SourceFormat(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_Load(This, feed) \
    ((This)->lpVtbl->Load(This, feed))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_LoadFromXml(This, feedDocument) \
    ((This)->lpVtbl->LoadFromXml(This, feedDocument))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationFeedFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationFeed
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationFeedFactory[] = L"Windows.Web.Syndication.ISyndicationFeedFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationFeed)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory* This,
        HSTRING title,
        HSTRING subtitle,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed** feed);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_CreateSyndicationFeed(This, title, subtitle, uri, feed) \
    ((This)->lpVtbl->CreateSyndicationFeed(This, title, subtitle, uri, feed))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeedFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationGenerator[] = L"Windows.Web.Syndication.ISyndicationGenerator";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Version)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_put_Version(This, value) \
    ((This)->lpVtbl->put_Version(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationGeneratorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationGeneratorFactory[] = L"Windows.Web.Syndication.ISyndicationGeneratorFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationGenerator)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory* This,
        HSTRING text,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGenerator** generator);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_CreateSyndicationGenerator(This, text, generator) \
    ((This)->lpVtbl->CreateSyndicationGenerator(This, text, generator))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationGeneratorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationItem
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationItem[] = L"Windows.Web.Syndication.ISyndicationItem";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Authors)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value);
    HRESULT (STDMETHODCALLTYPE* get_Categories)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationCategory** value);
    HRESULT (STDMETHODCALLTYPE* get_Contributors)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationPerson** value);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent** value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LastUpdatedTime)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_LastUpdatedTime)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Links)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationLink** value);
    HRESULT (STDMETHODCALLTYPE* get_PublishedDate)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_PublishedDate)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Rights)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Rights)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed** value);
    HRESULT (STDMETHODCALLTYPE* put_Source)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationFeed* value);
    HRESULT (STDMETHODCALLTYPE* get_Summary)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Summary)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* value);
    HRESULT (STDMETHODCALLTYPE* get_CommentsUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_CommentsUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_EditUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_EditMediaUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ETag)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ItemUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* Load)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        HSTRING item);
    HRESULT (STDMETHODCALLTYPE* LoadFromXml)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* itemDocument);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Authors(This, value) \
    ((This)->lpVtbl->get_Authors(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Categories(This, value) \
    ((This)->lpVtbl->get_Categories(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Contributors(This, value) \
    ((This)->lpVtbl->get_Contributors(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->get_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->put_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Links(This, value) \
    ((This)->lpVtbl->get_Links(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_PublishedDate(This, value) \
    ((This)->lpVtbl->get_PublishedDate(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_PublishedDate(This, value) \
    ((This)->lpVtbl->put_PublishedDate(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Rights(This, value) \
    ((This)->lpVtbl->get_Rights(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Rights(This, value) \
    ((This)->lpVtbl->put_Rights(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Source(This, value) \
    ((This)->lpVtbl->put_Source(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Summary(This, value) \
    ((This)->lpVtbl->get_Summary(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Summary(This, value) \
    ((This)->lpVtbl->put_Summary(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_CommentsUri(This, value) \
    ((This)->lpVtbl->get_CommentsUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_put_CommentsUri(This, value) \
    ((This)->lpVtbl->put_CommentsUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_EditUri(This, value) \
    ((This)->lpVtbl->get_EditUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_EditMediaUri(This, value) \
    ((This)->lpVtbl->get_EditMediaUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_ETag(This, value) \
    ((This)->lpVtbl->get_ETag(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_get_ItemUri(This, value) \
    ((This)->lpVtbl->get_ItemUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_Load(This, item) \
    ((This)->lpVtbl->Load(This, item))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_LoadFromXml(This, itemDocument) \
    ((This)->lpVtbl->LoadFromXml(This, itemDocument))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationItemFactory[] = L"Windows.Web.Syndication.ISyndicationItemFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationItem)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory* This,
        HSTRING title,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationContent* content,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItem** item);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_CreateSyndicationItem(This, title, content, uri, item) \
    ((This)->lpVtbl->CreateSyndicationItem(This, title, content, uri, item))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationLink
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationLink[] = L"Windows.Web.Syndication.ISyndicationLink";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Length)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_MediaType)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_MediaType)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Relationship)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Relationship)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceLanguage)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ResourceLanguage)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_Length(This, value) \
    ((This)->lpVtbl->put_Length(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_MediaType(This, value) \
    ((This)->lpVtbl->get_MediaType(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_MediaType(This, value) \
    ((This)->lpVtbl->put_MediaType(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_Relationship(This, value) \
    ((This)->lpVtbl->get_Relationship(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_Relationship(This, value) \
    ((This)->lpVtbl->put_Relationship(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_get_ResourceLanguage(This, value) \
    ((This)->lpVtbl->get_ResourceLanguage(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_put_ResourceLanguage(This, value) \
    ((This)->lpVtbl->put_ResourceLanguage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationLinkFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationLinkFactory[] = L"Windows.Web.Syndication.ISyndicationLinkFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationLink)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** link);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationLinkEx)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        HSTRING relationship,
        HSTRING title,
        HSTRING mediaType,
        UINT32 length,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLink** link);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_CreateSyndicationLink(This, uri, link) \
    ((This)->lpVtbl->CreateSyndicationLink(This, uri, link))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_CreateSyndicationLinkEx(This, uri, relationship, title, mediaType, length, link) \
    ((This)->lpVtbl->CreateSyndicationLinkEx(This, uri, relationship, title, mediaType, length, link))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationLinkFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationNode[] = L"Windows.Web.Syndication.ISyndicationNode";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NodeName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NodeName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_NodeNamespace)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NodeNamespace)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_NodeValue)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NodeValue)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Language)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_BaseUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_BaseUri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_AttributeExtensions)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        __FIVector_1_Windows__CWeb__CSyndication__CSyndicationAttribute** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementExtensions)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        __FIVector_1_Windows__CWeb__CSyndication__CISyndicationNode** value);
    HRESULT (STDMETHODCALLTYPE* GetXmlDocument)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode* This,
        enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationFormat format,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** xmlDocument);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_NodeName(This, value) \
    ((This)->lpVtbl->get_NodeName(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_put_NodeName(This, value) \
    ((This)->lpVtbl->put_NodeName(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_NodeNamespace(This, value) \
    ((This)->lpVtbl->get_NodeNamespace(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_put_NodeNamespace(This, value) \
    ((This)->lpVtbl->put_NodeNamespace(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_NodeValue(This, value) \
    ((This)->lpVtbl->get_NodeValue(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_put_NodeValue(This, value) \
    ((This)->lpVtbl->put_NodeValue(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_put_Language(This, value) \
    ((This)->lpVtbl->put_Language(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_BaseUri(This, value) \
    ((This)->lpVtbl->get_BaseUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_put_BaseUri(This, value) \
    ((This)->lpVtbl->put_BaseUri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_AttributeExtensions(This, value) \
    ((This)->lpVtbl->get_AttributeExtensions(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_get_ElementExtensions(This, value) \
    ((This)->lpVtbl->get_ElementExtensions(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_GetXmlDocument(This, format, xmlDocument) \
    ((This)->lpVtbl->GetXmlDocument(This, format, xmlDocument))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationNodeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationNodeFactory[] = L"Windows.Web.Syndication.ISyndicationNodeFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationNode)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory* This,
        HSTRING nodeName,
        HSTRING nodeNamespace,
        HSTRING nodeValue,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNode** node);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_CreateSyndicationNode(This, nodeName, nodeNamespace, nodeValue, node) \
    ((This)->lpVtbl->CreateSyndicationNode(This, nodeName, nodeNamespace, nodeValue, node))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationNodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationPerson
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationPerson
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationPerson[] = L"Windows.Web.Syndication.ISyndicationPerson";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Email)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Email)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_get_Email(This, value) \
    ((This)->lpVtbl->get_Email(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_put_Email(This, value) \
    ((This)->lpVtbl->put_Email(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationPersonFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationPerson
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationPersonFactory[] = L"Windows.Web.Syndication.ISyndicationPersonFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationPerson)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** person);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationPersonEx)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory* This,
        HSTRING name,
        HSTRING email,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPerson** person);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_CreateSyndicationPerson(This, name, person) \
    ((This)->lpVtbl->CreateSyndicationPerson(This, name, person))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_CreateSyndicationPersonEx(This, name, email, uri, person) \
    ((This)->lpVtbl->CreateSyndicationPersonEx(This, name, email, uri, person))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationPersonFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Web.Syndication.ISyndicationNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationText[] = L"Windows.Web.Syndication.ISyndicationText";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Xml)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* put_Xml)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationText* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_get_Xml(This, value) \
    ((This)->lpVtbl->get_Xml(This, value))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_put_Xml(This, value) \
    ((This)->lpVtbl->put_Xml(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationText;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Syndication.ISyndicationTextFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Syndication.SyndicationText
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Syndication_ISyndicationTextFactory[] = L"Windows.Web.Syndication.ISyndicationTextFactory";
typedef struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationText)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        HSTRING text,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** syndicationText);
    HRESULT (STDMETHODCALLTYPE* CreateSyndicationTextEx)(__x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory* This,
        HSTRING text,
        enum __x_ABI_CWindows_CWeb_CSyndication_CSyndicationTextType type,
        __x_ABI_CWindows_CWeb_CSyndication_CISyndicationText** syndicationText);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_CreateSyndicationText(This, text, syndicationText) \
    ((This)->lpVtbl->CreateSyndicationText(This, text, syndicationText))

#define __x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_CreateSyndicationTextEx(This, text, type, syndicationText) \
    ((This)->lpVtbl->CreateSyndicationTextEx(This, text, type, syndicationText))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CSyndication_CISyndicationTextFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationAttribute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationAttributeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationAttribute ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationAttribute_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationAttribute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationAttribute[] = L"Windows.Web.Syndication.SyndicationAttribute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationCategoryFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationCategory ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationCategory_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationCategory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationCategory[] = L"Windows.Web.Syndication.SyndicationCategory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationClient ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationClient_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationClient[] = L"Windows.Web.Syndication.SyndicationClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationText
 *    Windows.Web.Syndication.ISyndicationNode
 *    Windows.Web.Syndication.ISyndicationContent ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationContent[] = L"Windows.Web.Syndication.SyndicationContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Syndication.ISyndicationErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationError_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationError[] = L"Windows.Web.Syndication.SyndicationError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationFeed
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationFeedFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationFeed ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationFeed_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationFeed_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationFeed[] = L"Windows.Web.Syndication.SyndicationFeed";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationGeneratorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationGenerator ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationGenerator_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationGenerator[] = L"Windows.Web.Syndication.SyndicationGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationItem ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationItem_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationItem[] = L"Windows.Web.Syndication.SyndicationItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationLinkFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationLink ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationLink_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationLink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationLink[] = L"Windows.Web.Syndication.SyndicationLink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationNodeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationNode ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationNode_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationNode[] = L"Windows.Web.Syndication.SyndicationNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationPerson
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationPersonFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationPerson ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationPerson_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationPerson_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationPerson[] = L"Windows.Web.Syndication.SyndicationPerson";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Syndication.SyndicationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Syndication.ISyndicationTextFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Syndication.ISyndicationText ** Default Interface **
 *    Windows.Web.Syndication.ISyndicationNode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Syndication_SyndicationText_DEFINED
#define RUNTIMECLASS_Windows_Web_Syndication_SyndicationText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Syndication_SyndicationText[] = L"Windows.Web.Syndication.SyndicationText";
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
#endif // __windows2Eweb2Esyndication_p_h__

#endif // __windows2Eweb2Esyndication_h__
