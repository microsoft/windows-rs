
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
#ifndef __windows2Eweb2Ehttp2Efilters_h__
#define __windows2Eweb2Ehttp2Efilters_h__
#ifndef __windows2Eweb2Ehttp2Efilters_p_h__
#define __windows2Eweb2Ehttp2Efilters_p_h__


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

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

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
#include "Windows.Networking.Sockets.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Security.Cryptography.Certificates.h"
#include "Windows.System.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilter;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilter2;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2 ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter2

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilter3;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3 ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter3

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilter4;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4 ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter4

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilter5;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5 ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter5

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpBaseProtocolFilterStatics;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilterStatics

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpCacheControl;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl ABI::Windows::Web::Http::Filters::IHttpCacheControl

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpFilter;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter ABI::Windows::Web::Http::Filters::IHttpFilter

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    interface IHttpServerCustomValidationRequestedEventArgs;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs ABI::Windows::Web::Http::Filters::IHttpServerCustomValidationRequestedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef struct HttpProgress HttpProgress;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("beadb572-f9a3-5e93-b6ca-e311b65933fc"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpResponseMessage*, ABI::Windows::Web::Http::IHttpResponseMessage*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Http.HttpResponseMessage, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5d144364-77d7-5eca-8b09-936a69446652"))
IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpResponseMessage*, ABI::Windows::Web::Http::IHttpResponseMessage*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Http.HttpResponseMessage, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("68e4606a-76ec-5816-b2fe-a04ecde4126a"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpResponseMessage*, ABI::Windows::Web::Http::IHttpResponseMessage*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Http.HttpResponseMessage, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpResponseMessage*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class Certificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate ABI::Windows::Security::Cryptography::Certificates::ICertificate

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("676fc159-f15c-58bd-91a7-28f7e795c756"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c7d1423-e8fd-5a91-b55c-8bfbe7ac2d40"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    typedef enum ChainValidationResult : int ChainValidationResult;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8bcad2b7-0e3b-5eae-bf69-e1f6d9c888f8"))
IIterator<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IIterator_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2628f58f-3f02-54f2-808f-e1117709d6d0"))
IIterable<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IIterable_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("963f7013-77c2-51c5-8038-b5bcef633edb"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb383486-c2bc-5756-912d-6a708a07e5bd"))
IVectorView<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IVectorView_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#define DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7828cf7-4301-58d3-aab5-06e5eefcf79f"))
IVector<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> : IVector_impl<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Security.Cryptography.Certificates.ChainValidationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Security::Cryptography::Certificates::ChainValidationResult> __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t;
#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    class HttpBaseProtocolFilter;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    class HttpServerCustomValidationRequestedEventArgs;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ed0260c6-41f2-5a04-9a8b-2930d7ff8a9e"))
ITypedEventHandler<ABI::Windows::Web::Http::Filters::HttpBaseProtocolFilter*, ABI::Windows::Web::Http::Filters::HttpServerCustomValidationRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Filters::HttpBaseProtocolFilter*, ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::Filters::HttpServerCustomValidationRequestedEventArgs*, ABI::Windows::Web::Http::Filters::IHttpServerCustomValidationRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.Http.Filters.HttpBaseProtocolFilter, Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::Http::Filters::HttpBaseProtocolFilter*, ABI::Windows::Web::Http::Filters::HttpServerCustomValidationRequestedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Sockets {
                typedef enum SocketSslErrorSeverity : int SocketSslErrorSeverity;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

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
        namespace Web {
            namespace Http {
                class HttpCookieManager;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpCookieManager;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager ABI::Windows::Web::Http::IHttpCookieManager

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage ABI::Windows::Web::Http::IHttpRequestMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef enum HttpVersion : int HttpVersion;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    typedef enum HttpCacheReadBehavior : int HttpCacheReadBehavior;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    typedef enum HttpCacheWriteBehavior : int HttpCacheWriteBehavior;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    typedef enum HttpCookieUsageBehavior : int HttpCookieUsageBehavior;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    class HttpCacheControl;
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCacheReadBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    enum HttpCacheReadBehavior : int
                    {
                        HttpCacheReadBehavior_Default = 0,
                        HttpCacheReadBehavior_MostRecent = 1,
                        HttpCacheReadBehavior_OnlyFromCache = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        HttpCacheReadBehavior_NoCache = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    };
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCacheWriteBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    enum HttpCacheWriteBehavior : int
                    {
                        HttpCacheWriteBehavior_Default = 0,
                        HttpCacheWriteBehavior_NoCache = 1,
                    };
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCookieUsageBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    enum HttpCookieUsageBehavior : int
                    {
                        HttpCookieUsageBehavior_Default = 0,
                        HttpCookieUsageBehavior_NoCookies = 1,
                    };
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("71c89b09-e131-4b54-a53c-eb43ff37e9bb")
                    IHttpBaseProtocolFilter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllowAutoRedirect(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AllowAutoRedirect(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllowUI(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AllowUI(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutomaticDecompression(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AutomaticDecompression(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CacheControl(
                            ABI::Windows::Web::Http::Filters::IHttpCacheControl** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CookieManager(
                            ABI::Windows::Web::Http::IHttpCookieManager** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ClientCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ClientCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IgnorableServerCertificateErrors(
                            __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxConnectionsPerServer(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MaxConnectionsPerServer(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProxyCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ProxyCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UseProxy(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseProxy(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilter = __uuidof(IHttpBaseProtocolFilter);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter2[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter2";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("2ec30013-9427-4900-a017-fa7da3b5c9ae")
                    IHttpBaseProtocolFilter2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MaxVersion(
                            ABI::Windows::Web::Http::HttpVersion* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MaxVersion(
                            ABI::Windows::Web::Http::HttpVersion value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilter2 = __uuidof(IHttpBaseProtocolFilter2);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter3[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter3";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("d43f4d4c-bd42-43ae-8717-ad2c8f4b2937")
                    IHttpBaseProtocolFilter3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CookieUsageBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCookieUsageBehavior* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CookieUsageBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCookieUsageBehavior value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilter3 = __uuidof(IHttpBaseProtocolFilter3);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter4[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter4";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("9fe36ccf-2983-4893-941f-eb518ca8cef9")
                    IHttpBaseProtocolFilter4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_ServerCustomValidationRequested(
                            __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ServerCustomValidationRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearAuthenticationCache(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilter4 = __uuidof(IHttpBaseProtocolFilter4);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter5[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter5";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("416e4993-31e3-4816-bf09-e018ee8dc1f5")
                    IHttpBaseProtocolFilter5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_User(
                            ABI::Windows::System::IUser** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilter5 = __uuidof(IHttpBaseProtocolFilter5);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilterStatics[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("6d4dee0c-e908-494e-b5a3-1263c9b8242a")
                    IHttpBaseProtocolFilterStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateForUser(
                            ABI::Windows::System::IUser* user,
                            ABI::Windows::Web::Http::Filters::IHttpBaseProtocolFilter** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpBaseProtocolFilterStatics = __uuidof(IHttpBaseProtocolFilterStatics);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpCacheControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpCacheControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpCacheControl[] = L"Windows.Web.Http.Filters.IHttpCacheControl";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("c77e1cb4-3cea-4eb5-ac85-04e186e63ab7")
                    IHttpCacheControl : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ReadBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCacheReadBehavior* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ReadBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCacheReadBehavior value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WriteBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCacheWriteBehavior* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WriteBehavior(
                            ABI::Windows::Web::Http::Filters::HttpCacheWriteBehavior value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpCacheControl = __uuidof(IHttpCacheControl);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpFilter[] = L"Windows.Web.Http.Filters.IHttpFilter";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("a4cb6dd5-0902-439e-bfd7-e12552b165ce")
                    IHttpFilter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SendRequestAsync(
                            ABI::Windows::Web::Http::IHttpRequestMessage* request,
                            __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpFilter = __uuidof(IHttpFilter);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpServerCustomValidationRequestedEventArgs[] = L"Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Filters {
                    MIDL_INTERFACE("3165fe32-e7dd-48b7-a361-939c750e63cc")
                    IHttpServerCustomValidationRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                            ABI::Windows::Web::Http::IHttpRequestMessage** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerCertificate(
                            ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrorSeverity(
                            ABI::Windows::Networking::Sockets::SocketSslErrorSeverity* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerCertificateErrors(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerIntermediateCertificates(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Reject(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHttpServerCustomValidationRequestedEventArgs = __uuidof(IHttpServerCustomValidationRequestedEventArgs);
                } /* Filters */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter ** Default Interface **
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter2
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter3
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter4
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter5
 *    Windows.Web.Http.Filters.IHttpFilter
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpBaseProtocolFilter_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpBaseProtocolFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpBaseProtocolFilter[] = L"Windows.Web.Http.Filters.HttpBaseProtocolFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Filters.HttpCacheControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpCacheControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpCacheControl_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpCacheControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpCacheControl[] = L"Windows.Web.Http.Filters.HttpCacheControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs[] = L"Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2 __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3 __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4 __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5 __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress __x_ABI_CWindows_CWeb_CHttp_CHttpProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult;

typedef struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CChainValidationResult* items);

    END_INTERFACE
} __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl;

interface __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult
{
    CONST_VTBL struct __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* sender,
        __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity;

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion __x_ABI_CWindows_CWeb_CHttp_CHttpVersion;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheReadBehavior __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheReadBehavior;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheWriteBehavior __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheWriteBehavior;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCookieUsageBehavior __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCookieUsageBehavior;

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCacheReadBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheReadBehavior
{
    HttpCacheReadBehavior_Default = 0,
    HttpCacheReadBehavior_MostRecent = 1,
    HttpCacheReadBehavior_OnlyFromCache = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HttpCacheReadBehavior_NoCache = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCacheWriteBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheWriteBehavior
{
    HttpCacheWriteBehavior_Default = 0,
    HttpCacheWriteBehavior_NoCache = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.Filters.HttpCookieUsageBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCookieUsageBehavior
{
    HttpCookieUsageBehavior_Default = 0,
    HttpCookieUsageBehavior_NoCookies = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowAutoRedirect)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowAutoRedirect)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowUI)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowUI)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AutomaticDecompression)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutomaticDecompression)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CacheControl)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl** value);
    HRESULT (STDMETHODCALLTYPE* get_CookieManager)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager** value);
    HRESULT (STDMETHODCALLTYPE* get_ClientCertificate)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* put_ClientCertificate)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value);
    HRESULT (STDMETHODCALLTYPE* get_IgnorableServerCertificateErrors)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __FIVector_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxConnectionsPerServer)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxConnectionsPerServer)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ProxyCredential)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ProxyCredential)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCredential)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_ServerCredential)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_UseProxy)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseProxy)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_AllowAutoRedirect(This, value) \
    ((This)->lpVtbl->get_AllowAutoRedirect(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_AllowAutoRedirect(This, value) \
    ((This)->lpVtbl->put_AllowAutoRedirect(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_AllowUI(This, value) \
    ((This)->lpVtbl->get_AllowUI(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_AllowUI(This, value) \
    ((This)->lpVtbl->put_AllowUI(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_AutomaticDecompression(This, value) \
    ((This)->lpVtbl->get_AutomaticDecompression(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_AutomaticDecompression(This, value) \
    ((This)->lpVtbl->put_AutomaticDecompression(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_CacheControl(This, value) \
    ((This)->lpVtbl->get_CacheControl(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_CookieManager(This, value) \
    ((This)->lpVtbl->get_CookieManager(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_ClientCertificate(This, value) \
    ((This)->lpVtbl->get_ClientCertificate(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_ClientCertificate(This, value) \
    ((This)->lpVtbl->put_ClientCertificate(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_IgnorableServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_IgnorableServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_MaxConnectionsPerServer(This, value) \
    ((This)->lpVtbl->get_MaxConnectionsPerServer(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_MaxConnectionsPerServer(This, value) \
    ((This)->lpVtbl->put_MaxConnectionsPerServer(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_ProxyCredential(This, value) \
    ((This)->lpVtbl->get_ProxyCredential(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_ProxyCredential(This, value) \
    ((This)->lpVtbl->put_ProxyCredential(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_ServerCredential(This, value) \
    ((This)->lpVtbl->get_ServerCredential(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_ServerCredential(This, value) \
    ((This)->lpVtbl->put_ServerCredential(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_get_UseProxy(This, value) \
    ((This)->lpVtbl->get_UseProxy(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_put_UseProxy(This, value) \
    ((This)->lpVtbl->put_UseProxy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter2[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter2";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxVersion)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxVersion)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_get_MaxVersion(This, value) \
    ((This)->lpVtbl->get_MaxVersion(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_put_MaxVersion(This, value) \
    ((This)->lpVtbl->put_MaxVersion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter3[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter3";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CookieUsageBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCookieUsageBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_CookieUsageBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCookieUsageBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_get_CookieUsageBehavior(This, value) \
    ((This)->lpVtbl->get_CookieUsageBehavior(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_put_CookieUsageBehavior(This, value) \
    ((This)->lpVtbl->put_CookieUsageBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter4[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter4";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ServerCustomValidationRequested)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        __FITypedEventHandler_2_Windows__CWeb__CHttp__CFilters__CHttpBaseProtocolFilter_Windows__CWeb__CHttp__CFilters__CHttpServerCustomValidationRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ServerCustomValidationRequested)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* ClearAuthenticationCache)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4* This);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_add_ServerCustomValidationRequested(This, handler, token) \
    ((This)->lpVtbl->add_ServerCustomValidationRequested(This, handler, token))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_remove_ServerCustomValidationRequested(This, token) \
    ((This)->lpVtbl->remove_ServerCustomValidationRequested(This, token))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_ClearAuthenticationCache(This) \
    ((This)->lpVtbl->ClearAuthenticationCache(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilter5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilter5[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilter5";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpBaseProtocolFilterStatics[] = L"Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForUser)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilter** result);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStaticsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_CreateForUser(This, user, result) \
    ((This)->lpVtbl->CreateForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpBaseProtocolFilterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpCacheControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpCacheControl
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpCacheControl[] = L"Windows.Web.Http.Filters.IHttpCacheControl";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReadBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheReadBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_ReadBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheReadBehavior value);
    HRESULT (STDMETHODCALLTYPE* get_WriteBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheWriteBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_WriteBehavior)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CFilters_CHttpCacheWriteBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControlVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_get_ReadBehavior(This, value) \
    ((This)->lpVtbl->get_ReadBehavior(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_put_ReadBehavior(This, value) \
    ((This)->lpVtbl->put_ReadBehavior(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_get_WriteBehavior(This, value) \
    ((This)->lpVtbl->get_WriteBehavior(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_put_WriteBehavior(This, value) \
    ((This)->lpVtbl->put_WriteBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpCacheControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpFilter[] = L"Windows.Web.Http.Filters.IHttpFilter";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendRequestAsync)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilterVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_SendRequestAsync(This, request, operation) \
    ((This)->lpVtbl->SendRequestAsync(This, request, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_Filters_IHttpServerCustomValidationRequestedEventArgs[] = L"Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificate)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrorSeverity)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrors)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerIntermediateCertificates)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* Reject)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_get_ServerCertificate(This, value) \
    ((This)->lpVtbl->get_ServerCertificate(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_get_ServerCertificateErrorSeverity(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrorSeverity(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_get_ServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_get_ServerIntermediateCertificates(This, value) \
    ((This)->lpVtbl->get_ServerIntermediateCertificates(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_Reject(This) \
    ((This)->lpVtbl->Reject(This))

#define __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpServerCustomValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Web.Http.Filters.HttpBaseProtocolFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.Filters.IHttpBaseProtocolFilterStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter ** Default Interface **
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter2
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter3
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter4
 *    Windows.Web.Http.Filters.IHttpBaseProtocolFilter5
 *    Windows.Web.Http.Filters.IHttpFilter
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpBaseProtocolFilter_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpBaseProtocolFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpBaseProtocolFilter[] = L"Windows.Web.Http.Filters.HttpBaseProtocolFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Filters.HttpCacheControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpCacheControl ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpCacheControl_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpCacheControl_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpCacheControl[] = L"Windows.Web.Http.Filters.HttpCacheControl";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.Filters.IHttpServerCustomValidationRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_Filters_HttpServerCustomValidationRequestedEventArgs[] = L"Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs";
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
#endif // __windows2Eweb2Ehttp2Efilters_p_h__

#endif // __windows2Eweb2Ehttp2Efilters_h__
