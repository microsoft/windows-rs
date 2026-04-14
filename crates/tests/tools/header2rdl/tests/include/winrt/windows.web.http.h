
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
#ifndef __windows2Eweb2Ehttp_h__
#define __windows2Eweb2Ehttp_h__
#ifndef __windows2Eweb2Ehttp_p_h__
#define __windows2Eweb2Ehttp_p_h__


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

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Security.Cryptography.Certificates.h"
#include "Windows.Storage.Streams.h"
#include "Windows.Web.Http.Filters.h"
#include "Windows.Web.Http.Headers.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpBufferContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory ABI::Windows::Web::Http::IHttpBufferContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpClient;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient ABI::Windows::Web::Http::IHttpClient

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpClient2;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2 ABI::Windows::Web::Http::IHttpClient2

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpClient3;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3 ABI::Windows::Web::Http::IHttpClient3

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpClientFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory ABI::Windows::Web::Http::IHttpClientFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent ABI::Windows::Web::Http::IHttpContent

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpCookie;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie ABI::Windows::Web::Http::IHttpCookie

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpCookieFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory ABI::Windows::Web::Http::IHttpCookieFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpFormUrlEncodedContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory ABI::Windows::Web::Http::IHttpFormUrlEncodedContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpGetBufferResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult ABI::Windows::Web::Http::IHttpGetBufferResult

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpGetInputStreamResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult ABI::Windows::Web::Http::IHttpGetInputStreamResult

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpGetStringResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult ABI::Windows::Web::Http::IHttpGetStringResult

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMethod;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod ABI::Windows::Web::Http::IHttpMethod

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMethodFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory ABI::Windows::Web::Http::IHttpMethodFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMethodStatics;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics ABI::Windows::Web::Http::IHttpMethodStatics

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMultipartContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent ABI::Windows::Web::Http::IHttpMultipartContent

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMultipartContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory ABI::Windows::Web::Http::IHttpMultipartContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMultipartFormDataContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent ABI::Windows::Web::Http::IHttpMultipartFormDataContent

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpMultipartFormDataContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory ABI::Windows::Web::Http::IHttpMultipartFormDataContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage2;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2 ABI::Windows::Web::Http::IHttpRequestMessage2

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessageFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory ABI::Windows::Web::Http::IHttpRequestMessageFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult ABI::Windows::Web::Http::IHttpRequestResult

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessageFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory ABI::Windows::Web::Http::IHttpResponseMessageFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpStreamContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory ABI::Windows::Web::Http::IHttpStreamContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpStringContentFactory;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory ABI::Windows::Web::Http::IHttpStringContentFactory

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpTransportInformation;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation ABI::Windows::Web::Http::IHttpTransportInformation

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd75eebe-e7b5-5af6-8415-a4b9c9045202"))
IAsyncOperationWithProgressCompletedHandler<HSTRING, UINT64> : IAsyncOperationWithProgressCompletedHandler_impl<HSTRING, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<String, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<HSTRING, UINT64> __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_HSTRING_UINT64_USE
#define DEF___FIAsyncOperationWithProgress_2_HSTRING_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c8bbcb29-6b64-5ce2-a831-038f6e02199e"))
IAsyncOperationWithProgress<HSTRING, UINT64> : IAsyncOperationWithProgress_impl<HSTRING, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<String, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<HSTRING, UINT64> __FIAsyncOperationWithProgress_2_HSTRING_UINT64_t;
#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_HSTRING_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_HSTRING_UINT64_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_HSTRING_UINT64_USE
#define DEF___FIAsyncOperationProgressHandler_2_HSTRING_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("14da7de7-40df-5d4c-823f-cf310625ad39"))
IAsyncOperationProgressHandler<HSTRING, UINT64> : IAsyncOperationProgressHandler_impl<HSTRING, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<String, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<HSTRING, UINT64> __FIAsyncOperationProgressHandler_2_HSTRING_UINT64_t;
#define __FIAsyncOperationProgressHandler_2_HSTRING_UINT64 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_HSTRING_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_HSTRING_UINT64_USE */


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

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("98ab9acb-38db-588f-a5f9-9f484b2200cd"))
IAsyncOperationWithProgressCompletedHandler<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<HSTRING, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<String, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("91ecbe45-e889-5518-bd8d-c5bde163109b"))
IAsyncOperationWithProgress<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<HSTRING, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<String, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cf0a03f6-a80a-5b46-9c80-f4ad9ed6e2d6"))
IAsyncOperationProgressHandler<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<HSTRING, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<String, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<HSTRING, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d2024e41-5500-5b5a-ba46-cb7009596a2f"))
IAsyncOperationWithProgressCompletedHandler<UINT64, UINT64> : IAsyncOperationWithProgressCompletedHandler_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<UINT64, UINT64> __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8f1db6e3-6556-5516-825c-1021ee27cd0c"))
IAsyncOperationWithProgress<UINT64, UINT64> : IAsyncOperationWithProgress_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<UINT64, UINT64> __FIAsyncOperationWithProgress_2_UINT64_UINT64_t;
#define __FIAsyncOperationWithProgress_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_UINT64_UINT64_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE
#define DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ffb2b65d-4120-5d13-826d-107851e6bb1c"))
IAsyncOperationProgressHandler<UINT64, UINT64> : IAsyncOperationProgressHandler_impl<UINT64, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<UInt64, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<UINT64, UINT64> __FIAsyncOperationProgressHandler_2_UINT64_UINT64_t;
#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_UINT64_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_UINT64_UINT64_USE */


#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abc81235-39c7-59bf-9948-2d14a93d40fd"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT64> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Storage.Streams.IBuffer, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT64> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad960e7f-d73b-56e4-a58c-6ec7678cfd88"))
IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, UINT64> : IAsyncOperationWithProgress_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Storage.Streams.IBuffer, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, UINT64> __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t;
#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d17f5eb6-b422-5e26-a817-7e0fd08f75d5"))
IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT64> : IAsyncOperationProgressHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Storage.Streams.IBuffer, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, UINT64> __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b0cf2f85-6992-52be-8f0b-93964b14d963"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Storage.Streams.IBuffer, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88d9bb75-afb4-5f32-9d7e-d3bf3785354c"))
IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Storage.Streams.IBuffer, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9eb2b852-e019-5440-8f88-0dd7d56fea47"))
IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Storage.Streams.IBuffer, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IBuffer*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8db69706-3dd1-5a28-986a-93be0776d9c3"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*, UINT64> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Storage::Streams::IInputStream*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Storage.Streams.IInputStream, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*, UINT64> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("455aa601-f13e-5dee-b9cb-16b531996327"))
IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IInputStream*, UINT64> : IAsyncOperationWithProgress_impl<ABI::Windows::Storage::Streams::IInputStream*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Storage.Streams.IInputStream, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IInputStream*, UINT64> __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t;
#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f9b2e7f6-762f-50db-95dd-7f6c6ec47090"))
IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IInputStream*, UINT64> : IAsyncOperationProgressHandler_impl<ABI::Windows::Storage::Streams::IInputStream*, UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Storage.Streams.IInputStream, UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IInputStream*, UINT64> __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("504a34ec-5499-5a16-bffc-3ccb64a3547a"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Storage.Streams.IInputStream, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0b97c784-df17-571f-8337-447dff068a9c"))
IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Storage.Streams.IInputStream, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("04682e89-6e8b-54b1-a466-432e130cf9a6"))
IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Storage.Streams.IInputStream, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Storage::Streams::IInputStream*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpGetBufferResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6bec534a-1901-5fa2-9686-9a510f6b1217"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetBufferResult*, ABI::Windows::Web::Http::IHttpGetBufferResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Http.HttpGetBufferResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b128fbc4-19c0-5fe1-aec6-d2e64bd22862"))
IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetBufferResult*, ABI::Windows::Web::Http::IHttpGetBufferResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Http.HttpGetBufferResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43624279-9a68-5052-a9de-19569b54818a"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetBufferResult*, ABI::Windows::Web::Http::IHttpGetBufferResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Http.HttpGetBufferResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetBufferResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpGetInputStreamResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("789b1519-4ae1-5475-bb01-6734cad478ff"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetInputStreamResult*, ABI::Windows::Web::Http::IHttpGetInputStreamResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Http.HttpGetInputStreamResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e1ce5ac-c10b-5d09-ad43-f4ddeada857a"))
IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetInputStreamResult*, ABI::Windows::Web::Http::IHttpGetInputStreamResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Http.HttpGetInputStreamResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5d399e45-63aa-52f7-bbb6-c718ca64bd2a"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetInputStreamResult*, ABI::Windows::Web::Http::IHttpGetInputStreamResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Http.HttpGetInputStreamResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetInputStreamResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpGetStringResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("625df3b5-bff7-517a-9359-1dc28fb0f586"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetStringResult*, ABI::Windows::Web::Http::IHttpGetStringResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Http.HttpGetStringResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7382f299-bbbd-5bd3-b143-8887c627929b"))
IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetStringResult*, ABI::Windows::Web::Http::IHttpGetStringResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Http.HttpGetStringResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43516920-bbfc-5ba6-9d59-8af34fc97b4e"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpGetStringResult*, ABI::Windows::Web::Http::IHttpGetStringResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Http.HttpGetStringResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpGetStringResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestResult;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fdfc3d8f-08bb-5d82-bfde-ae092e7e3fe1"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpRequestResult*, ABI::Windows::Web::Http::IHttpRequestResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Web.Http.HttpRequestResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("97388336-2eac-5d76-b228-d32ef9a38175"))
IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpRequestResult*, ABI::Windows::Web::Http::IHttpRequestResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Web.Http.HttpRequestResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("167f8eda-2a7a-56a2-bba3-76d78e24f13d"))
IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpRequestResult*, ABI::Windows::Web::Http::IHttpRequestResult*>, struct ABI::Windows::Web::Http::HttpProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Web.Http.HttpRequestResult, Windows.Web.Http.HttpProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Web::Http::HttpRequestResult*, struct ABI::Windows::Web::Http::HttpProgress> __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
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


#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



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

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpCookie;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_USE
#define DEF___FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("626bc177-8403-5030-a88c-7485cc89d730"))
IIterator<ABI::Windows::Web::Http::HttpCookie*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpCookie*, ABI::Windows::Web::Http::IHttpCookie*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Http.HttpCookie>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Http::HttpCookie*> __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_t;
#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_USE
#define DEF___FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0eb9fa36-88de-590d-8ea0-b613d0ab015f"))
IIterable<ABI::Windows::Web::Http::HttpCookie*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpCookie*, ABI::Windows::Web::Http::IHttpCookie*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.HttpCookie>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Http::HttpCookie*> __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_t;
#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_USE
#define DEF___FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("59f44f31-695e-5af7-a3c5-85c01939cec8"))
IIterator<ABI::Windows::Web::Http::IHttpContent*> : IIterator_impl<ABI::Windows::Web::Http::IHttpContent*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.Http.IHttpContent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::Http::IHttpContent*> __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_t;
#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_USE
#define DEF___FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f102157f-b482-5736-9d12-c683bc494942"))
IIterable<ABI::Windows::Web::Http::IHttpContent*> : IIterable_impl<ABI::Windows::Web::Http::IHttpContent*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.IHttpContent>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::Http::IHttpContent*> __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_t;
#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMap_2_HSTRING_IInspectable_USE
#define DEF___FIMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1b0d3570-0877-5ec2-8a2c-3b9539506aca"))
IMap<HSTRING, IInspectable*> : IMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, IInspectable*> __FIMap_2_HSTRING_IInspectable_t;
#define __FIMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_IInspectable_USE */


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

#ifndef DEF___FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_USE
#define DEF___FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0064c4f6-3fca-5823-9d92-86c40b28adbc"))
IVectorView<ABI::Windows::Web::Http::HttpCookie*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpCookie*, ABI::Windows::Web::Http::IHttpCookie*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.Http.HttpCookie>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::Http::HttpCookie*> __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_t;
#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_UINT64_USE
#define DEF___FIReference_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6755e376-53bb-568b-a11d-17239868309e"))
IReference<UINT64> : IReference_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT64> __FIReference_1_UINT64_t;
#define __FIReference_1_UINT64 ABI::Windows::Foundation::__FIReference_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT64_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IStringable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIStringable ABI::Windows::Foundation::IStringable

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

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
        namespace Networking {
            namespace Sockets {
                typedef enum SocketSslErrorSeverity : int SocketSslErrorSeverity;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                typedef enum UnicodeEncoding : int UnicodeEncoding;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    class HttpContentHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    interface IHttpContentHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection ABI::Windows::Web::Http::Headers::IHttpContentHeaderCollection

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    class HttpRequestHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    interface IHttpRequestHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection ABI::Windows::Web::Http::Headers::IHttpRequestHeaderCollection

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    class HttpResponseHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                namespace Headers {
                    interface IHttpResponseHeaderCollection;
                } /* Headers */
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection ABI::Windows::Web::Http::Headers::IHttpResponseHeaderCollection

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef enum HttpCompletionOption : int HttpCompletionOption;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef enum HttpProgressStage : int HttpProgressStage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef enum HttpResponseMessageSource : int HttpResponseMessageSource;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                typedef enum HttpStatusCode : int HttpStatusCode;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

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
                class HttpBufferContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpClient;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpCookieCollection;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpFormUrlEncodedContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpMethod;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpMultipartContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpMultipartFormDataContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpStreamContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpStringContent;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpTransportInformation;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.Http.HttpCompletionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                enum HttpCompletionOption : int
                {
                    HttpCompletionOption_ResponseContentRead = 0,
                    HttpCompletionOption_ResponseHeadersRead = 1,
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpProgressStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                enum HttpProgressStage : int
                {
                    HttpProgressStage_None = 0,
                    HttpProgressStage_DetectingProxy = 10,
                    HttpProgressStage_ResolvingName = 20,
                    HttpProgressStage_ConnectingToServer = 30,
                    HttpProgressStage_NegotiatingSsl = 40,
                    HttpProgressStage_SendingHeaders = 50,
                    HttpProgressStage_SendingContent = 60,
                    HttpProgressStage_WaitingForResponse = 70,
                    HttpProgressStage_ReceivingHeaders = 80,
                    HttpProgressStage_ReceivingContent = 90,
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpResponseMessageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                enum HttpResponseMessageSource : int
                {
                    HttpResponseMessageSource_None = 0,
                    HttpResponseMessageSource_Cache = 1,
                    HttpResponseMessageSource_Network = 2,
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpStatusCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                enum HttpStatusCode : int
                {
                    HttpStatusCode_None = 0,
                    HttpStatusCode_Continue = 100,
                    HttpStatusCode_SwitchingProtocols = 101,
                    HttpStatusCode_Processing = 102,
                    HttpStatusCode_Ok = 200,
                    HttpStatusCode_Created = 201,
                    HttpStatusCode_Accepted = 202,
                    HttpStatusCode_NonAuthoritativeInformation = 203,
                    HttpStatusCode_NoContent = 204,
                    HttpStatusCode_ResetContent = 205,
                    HttpStatusCode_PartialContent = 206,
                    HttpStatusCode_MultiStatus = 207,
                    HttpStatusCode_AlreadyReported = 208,
                    HttpStatusCode_IMUsed = 226,
                    HttpStatusCode_MultipleChoices = 300,
                    HttpStatusCode_MovedPermanently = 301,
                    HttpStatusCode_Found = 302,
                    HttpStatusCode_SeeOther = 303,
                    HttpStatusCode_NotModified = 304,
                    HttpStatusCode_UseProxy = 305,
                    HttpStatusCode_TemporaryRedirect = 307,
                    HttpStatusCode_PermanentRedirect = 308,
                    HttpStatusCode_BadRequest = 400,
                    HttpStatusCode_Unauthorized = 401,
                    HttpStatusCode_PaymentRequired = 402,
                    HttpStatusCode_Forbidden = 403,
                    HttpStatusCode_NotFound = 404,
                    HttpStatusCode_MethodNotAllowed = 405,
                    HttpStatusCode_NotAcceptable = 406,
                    HttpStatusCode_ProxyAuthenticationRequired = 407,
                    HttpStatusCode_RequestTimeout = 408,
                    HttpStatusCode_Conflict = 409,
                    HttpStatusCode_Gone = 410,
                    HttpStatusCode_LengthRequired = 411,
                    HttpStatusCode_PreconditionFailed = 412,
                    HttpStatusCode_RequestEntityTooLarge = 413,
                    HttpStatusCode_RequestUriTooLong = 414,
                    HttpStatusCode_UnsupportedMediaType = 415,
                    HttpStatusCode_RequestedRangeNotSatisfiable = 416,
                    HttpStatusCode_ExpectationFailed = 417,
                    HttpStatusCode_UnprocessableEntity = 422,
                    HttpStatusCode_Locked = 423,
                    HttpStatusCode_FailedDependency = 424,
                    HttpStatusCode_UpgradeRequired = 426,
                    HttpStatusCode_PreconditionRequired = 428,
                    HttpStatusCode_TooManyRequests = 429,
                    HttpStatusCode_RequestHeaderFieldsTooLarge = 431,
                    HttpStatusCode_InternalServerError = 500,
                    HttpStatusCode_NotImplemented = 501,
                    HttpStatusCode_BadGateway = 502,
                    HttpStatusCode_ServiceUnavailable = 503,
                    HttpStatusCode_GatewayTimeout = 504,
                    HttpStatusCode_HttpVersionNotSupported = 505,
                    HttpStatusCode_VariantAlsoNegotiates = 506,
                    HttpStatusCode_InsufficientStorage = 507,
                    HttpStatusCode_LoopDetected = 508,
                    HttpStatusCode_NotExtended = 510,
                    HttpStatusCode_NetworkAuthenticationRequired = 511,
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpVersion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                enum HttpVersion : int
                {
                    HttpVersion_None = 0,
                    HttpVersion_Http10 = 1,
                    HttpVersion_Http11 = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    HttpVersion_Http20 = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                struct HttpProgress
                {
                    ABI::Windows::Web::Http::HttpProgressStage Stage;
                    UINT64 BytesSent;
                    __FIReference_1_UINT64* TotalBytesToSend;
                    UINT64 BytesReceived;
                    __FIReference_1_UINT64* TotalBytesToReceive;
                    UINT32 Retries;
                };
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpBufferContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpBufferContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpBufferContentFactory[] = L"Windows.Web.Http.IHttpBufferContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("bc20c193-c41f-4ff7-9123-6435736eadc2")
                IHttpBufferContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBuffer(
                        ABI::Windows::Storage::Streams::IBuffer* content,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromBufferWithOffset(
                        ABI::Windows::Storage::Streams::IBuffer* content,
                        UINT32 offset,
                        UINT32 count,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpBufferContentFactory = __uuidof(IHttpBufferContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient[] = L"Windows.Web.Http.IHttpClient";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("7fda1151-3574-4880-a8ba-e6b1e0061f3d")
                IHttpClient : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetWithOptionAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::HttpCompletionOption completionOption,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBufferAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInputStreamAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetStringAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PostAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::IHttpContent* content,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PutAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::IHttpContent* content,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendRequestAsync(
                        ABI::Windows::Web::Http::IHttpRequestMessage* request,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendRequestWithOptionAsync(
                        ABI::Windows::Web::Http::IHttpRequestMessage* request,
                        ABI::Windows::Web::Http::HttpCompletionOption completionOption,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultRequestHeaders(
                        ABI::Windows::Web::Http::Headers::IHttpRequestHeaderCollection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpClient = __uuidof(IHttpClient);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpClient2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient2[] = L"Windows.Web.Http.IHttpClient2";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("cdd83348-e8b7-4cec-b1b0-dc455fe72c92")
                IHttpClient2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryDeleteAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetAsync2(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::HttpCompletionOption completionOption,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetBufferAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetInputStreamAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryGetStringAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryPostAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::IHttpContent* content,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryPutAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::IHttpContent* content,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendRequestAsync(
                        ABI::Windows::Web::Http::IHttpRequestMessage* request,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySendRequestAsync2(
                        ABI::Windows::Web::Http::IHttpRequestMessage* request,
                        ABI::Windows::Web::Http::HttpCompletionOption completionOption,
                        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpClient2 = __uuidof(IHttpClient2);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpClient3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient3[] = L"Windows.Web.Http.IHttpClient3";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("1172fd01-9899-4194-963f-8f9d72a7ec15")
                IHttpClient3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultPrivacyAnnotation(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultPrivacyAnnotation(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpClient3 = __uuidof(IHttpClient3);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient3;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Web.Http.IHttpClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClientFactory[] = L"Windows.Web.Http.IHttpClientFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("c30c4eca-e3fa-4f99-afb4-63cc65009462")
                IHttpClientFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Web::Http::Filters::IHttpFilter* filter,
                        ABI::Windows::Web::Http::IHttpClient** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpClientFactory = __uuidof(IHttpClientFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpContent[] = L"Windows.Web.Http.IHttpContent";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("6b14a441-fba7-4bd2-af0a-839de7c295da")
                IHttpContent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        ABI::Windows::Web::Http::Headers::IHttpContentHeaderCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BufferAllAsync(
                        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadAsBufferAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadAsInputStreamAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadAsStringAsync(
                        __FIAsyncOperationWithProgress_2_HSTRING_UINT64** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryComputeLength(
                        UINT64* length,
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE WriteToStreamAsync(
                        ABI::Windows::Storage::Streams::IOutputStream* outputStream,
                        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpContent = __uuidof(IHttpContent);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookie
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookie
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookie[] = L"Windows.Web.Http.IHttpCookie";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("1f5488e2-cc2d-4779-86a7-88f10687d249")
                IHttpCookie : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Domain(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Path(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Expires(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Expires(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HttpOnly(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HttpOnly(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Secure(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Secure(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpCookie = __uuidof(IHttpCookie);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookie;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookieFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookie
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookieFactory[] = L"Windows.Web.Http.IHttpCookieFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("6a0585a9-931c-4cd1-a96d-c21701785c5f")
                IHttpCookieFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING name,
                        HSTRING domain,
                        HSTRING path,
                        ABI::Windows::Web::Http::IHttpCookie** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpCookieFactory = __uuidof(IHttpCookieFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookieManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookieManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookieManager[] = L"Windows.Web.Http.IHttpCookieManager";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("7a431780-cd4f-4e57-a84a-5b0a53d6bb96")
                IHttpCookieManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetCookie(
                        ABI::Windows::Web::Http::IHttpCookie* cookie,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetCookieWithThirdParty(
                        ABI::Windows::Web::Http::IHttpCookie* cookie,
                        boolean thirdParty,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteCookie(
                        ABI::Windows::Web::Http::IHttpCookie* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCookies(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpCookieManager = __uuidof(IHttpCookieManager);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpFormUrlEncodedContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpFormUrlEncodedContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpFormUrlEncodedContentFactory[] = L"Windows.Web.Http.IHttpFormUrlEncodedContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("43f0138c-2f73-4302-b5f3-eae9238a5e01")
                IHttpFormUrlEncodedContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* content,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpFormUrlEncodedContentFactory = __uuidof(IHttpFormUrlEncodedContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpGetBufferResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetBufferResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetBufferResult[] = L"Windows.Web.Http.IHttpGetBufferResult";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("53d08e7c-e209-404e-9a49-742d8236fd3a")
                IHttpGetBufferResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResponseMessage(
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpGetBufferResult = __uuidof(IHttpGetBufferResult);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpGetInputStreamResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetInputStreamResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetInputStreamResult[] = L"Windows.Web.Http.IHttpGetInputStreamResult";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("d5d63463-13aa-4ee0-be95-a0c39fe91203")
                IHttpGetInputStreamResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResponseMessage(
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpGetInputStreamResult = __uuidof(IHttpGetInputStreamResult);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpGetStringResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetStringResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetStringResult[] = L"Windows.Web.Http.IHttpGetStringResult";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("9bac466d-8509-4775-b16d-8953f47a7f5f")
                IHttpGetStringResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResponseMessage(
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpGetStringResult = __uuidof(IHttpGetStringResult);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethod[] = L"Windows.Web.Http.IHttpMethod";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("728d4022-700d-4fe0-afa5-40299c58dbfd")
                IHttpMethod : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Method(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMethod = __uuidof(IHttpMethod);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethod;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMethodFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethodFactory[] = L"Windows.Web.Http.IHttpMethodFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("3c51d10d-36d7-40f8-a86d-e759caf2f83f")
                IHttpMethodFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING method,
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMethodFactory = __uuidof(IHttpMethodFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMethodStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethodStatics[] = L"Windows.Web.Http.IHttpMethodStatics";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("64d171f0-d99a-4153-8dc6-d68cc4cce317")
                IHttpMethodStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Delete(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Get(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Head(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Patch(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Post(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Put(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMethodStatics = __uuidof(IHttpMethodStatics);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartContent[] = L"Windows.Web.Http.IHttpMultipartContent";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("df916aff-9926-4ac9-aaf1-e0d04ef09bb9")
                IHttpMultipartContent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Add(
                        ABI::Windows::Web::Http::IHttpContent* content
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMultipartContent = __uuidof(IHttpMultipartContent);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartContentFactory[] = L"Windows.Web.Http.IHttpMultipartContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("7eb42e62-0222-4f20-b372-47d5db5d33b4")
                IHttpMultipartContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSubtype(
                        HSTRING subtype,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSubtypeAndBoundary(
                        HSTRING subtype,
                        HSTRING boundary,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMultipartContentFactory = __uuidof(IHttpMultipartContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartFormDataContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartFormDataContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartFormDataContent[] = L"Windows.Web.Http.IHttpMultipartFormDataContent";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("64d337e2-e967-4624-b6d1-cf74604a4a42")
                IHttpMultipartFormDataContent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Add(
                        ABI::Windows::Web::Http::IHttpContent* content
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddWithName(
                        ABI::Windows::Web::Http::IHttpContent* content,
                        HSTRING name
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddWithNameAndFileName(
                        ABI::Windows::Web::Http::IHttpContent* content,
                        HSTRING name,
                        HSTRING fileName
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMultipartFormDataContent = __uuidof(IHttpMultipartFormDataContent);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartFormDataContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartFormDataContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartFormDataContentFactory[] = L"Windows.Web.Http.IHttpMultipartFormDataContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("a04d7311-5017-4622-93a8-49b24a4fcbfc")
                IHttpMultipartFormDataContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithBoundary(
                        HSTRING boundary,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpMultipartFormDataContentFactory = __uuidof(IHttpMultipartFormDataContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessage[] = L"Windows.Web.Http.IHttpRequestMessage";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("f5762b3c-74d4-4811-b5dc-9f8b4e2f9abf")
                IHttpRequestMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Content(
                        ABI::Windows::Web::Http::IHttpContent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        ABI::Windows::Web::Http::Headers::IHttpRequestHeaderCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Method(
                        ABI::Windows::Web::Http::IHttpMethod** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Method(
                        ABI::Windows::Web::Http::IHttpMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMap_2_HSTRING_IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportInformation(
                        ABI::Windows::Web::Http::IHttpTransportInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpRequestMessage = __uuidof(IHttpRequestMessage);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessage2[] = L"Windows.Web.Http.IHttpRequestMessage2";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("c3c60489-62c2-4a3f-9554-226e7c60bd96")
                IHttpRequestMessage2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PrivacyAnnotation(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrivacyAnnotation(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpRequestMessage2 = __uuidof(IHttpRequestMessage2);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessageFactory[] = L"Windows.Web.Http.IHttpRequestMessageFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("5bac994e-3886-412e-aec3-52ec7f25616f")
                IHttpRequestMessageFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Web::Http::IHttpMethod* method,
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpRequestMessageFactory = __uuidof(IHttpRequestMessageFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestResult[] = L"Windows.Web.Http.IHttpRequestResult";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("6acf4da8-b5eb-4a35-a902-4217fbe820c5")
                IHttpRequestResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResponseMessage(
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpRequestResult = __uuidof(IHttpRequestResult);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpResponseMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpResponseMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpResponseMessage[] = L"Windows.Web.Http.IHttpResponseMessage";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("fee200fb-8664-44e0-95d9-42696199bffc")
                IHttpResponseMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Content(
                        ABI::Windows::Web::Http::IHttpContent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        ABI::Windows::Web::Http::Headers::IHttpResponseHeaderCollection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSuccessStatusCode(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReasonPhrase(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReasonPhrase(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Web::Http::HttpResponseMessageSource* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Source(
                        ABI::Windows::Web::Http::HttpResponseMessageSource value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StatusCode(
                        ABI::Windows::Web::Http::HttpStatusCode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StatusCode(
                        ABI::Windows::Web::Http::HttpStatusCode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        ABI::Windows::Web::Http::HttpVersion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Version(
                        ABI::Windows::Web::Http::HttpVersion value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnsureSuccessStatusCode(
                        ABI::Windows::Web::Http::IHttpResponseMessage** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpResponseMessage = __uuidof(IHttpResponseMessage);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpResponseMessageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpResponseMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpResponseMessageFactory[] = L"Windows.Web.Http.IHttpResponseMessageFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("52a8af99-f095-43da-b60f-7cfc2bc7ea2f")
                IHttpResponseMessageFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Web::Http::HttpStatusCode statusCode,
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpResponseMessageFactory = __uuidof(IHttpResponseMessageFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpStreamContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpStreamContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpStreamContentFactory[] = L"Windows.Web.Http.IHttpStreamContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("f3e64d9d-f725-407e-942f-0eda189809f4")
                IHttpStreamContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromInputStream(
                        ABI::Windows::Storage::Streams::IInputStream* content,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpStreamContentFactory = __uuidof(IHttpStreamContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpStringContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpStringContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpStringContentFactory[] = L"Windows.Web.Http.IHttpStringContentFactory";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("46649d5b-2e93-48eb-8e61-19677878e57f")
                IHttpStringContentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromString(
                        HSTRING content,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromStringWithEncoding(
                        HSTRING content,
                        ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromStringWithEncodingAndMediaType(
                        HSTRING content,
                        ABI::Windows::Storage::Streams::UnicodeEncoding encoding,
                        HSTRING mediaType,
                        ABI::Windows::Web::Http::IHttpContent** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHttpStringContentFactory = __uuidof(IHttpStringContentFactory);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpTransportInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpTransportInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpTransportInformation[] = L"Windows.Web.Http.IHttpTransportInformation";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                MIDL_INTERFACE("70127198-c6a7-4ed0-833a-83fd8b8f178d")
                IHttpTransportInformation : public IInspectable
                {
                public:
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
                };

                MIDL_CONST_ID IID& IID_IHttpTransportInformation = __uuidof(IHttpTransportInformation);
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpBufferContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpBufferContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpBufferContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpBufferContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpBufferContent[] = L"Windows.Web.Http.HttpBufferContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpClient ** Default Interface **
 *    Windows.Web.Http.IHttpClient2
 *    Windows.Web.Http.IHttpClient3
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpClient_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpClient[] = L"Windows.Web.Http.HttpClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookie
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpCookieFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpCookie ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookie_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookie_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookie[] = L"Windows.Web.Http.HttpCookie";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookieCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Web.Http.HttpCookie> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.HttpCookie>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookieCollection_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookieCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookieCollection[] = L"Windows.Web.Http.HttpCookieCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookieManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpCookieManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookieManager_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookieManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookieManager[] = L"Windows.Web.Http.HttpCookieManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpFormUrlEncodedContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpFormUrlEncodedContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpFormUrlEncodedContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpFormUrlEncodedContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpFormUrlEncodedContent[] = L"Windows.Web.Http.HttpFormUrlEncodedContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpGetBufferResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetBufferResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetBufferResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetBufferResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetBufferResult[] = L"Windows.Web.Http.HttpGetBufferResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpGetInputStreamResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetInputStreamResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetInputStreamResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetInputStreamResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetInputStreamResult[] = L"Windows.Web.Http.HttpGetInputStreamResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpGetStringResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetStringResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetStringResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetStringResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetStringResult[] = L"Windows.Web.Http.HttpGetStringResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpMethodFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.IHttpMethodStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMethod ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMethod_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMethod_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMethod[] = L"Windows.Web.Http.HttpMethod";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpMultipartContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpMultipartContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMultipartContent
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.IHttpContent>
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMultipartContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMultipartContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMultipartContent[] = L"Windows.Web.Http.HttpMultipartContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpMultipartFormDataContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpMultipartFormDataContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMultipartFormDataContent
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.IHttpContent>
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMultipartFormDataContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMultipartFormDataContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMultipartFormDataContent[] = L"Windows.Web.Http.HttpMultipartFormDataContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpRequestMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpRequestMessageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpRequestMessage ** Default Interface **
 *    Windows.Web.Http.IHttpRequestMessage2
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpRequestMessage_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpRequestMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpRequestMessage[] = L"Windows.Web.Http.HttpRequestMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpRequestResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpRequestResult[] = L"Windows.Web.Http.HttpRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpResponseMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpResponseMessageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpResponseMessage ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpResponseMessage_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpResponseMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpResponseMessage[] = L"Windows.Web.Http.HttpResponseMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpStreamContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpStreamContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpStreamContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpStreamContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpStreamContent[] = L"Windows.Web.Http.HttpStreamContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpStringContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpStringContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpStringContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpStringContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpStringContent[] = L"Windows.Web.Http.HttpStringContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpTransportInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpTransportInformation ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpTransportInformation_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpTransportInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpTransportInformation[] = L"Windows.Web.Http.HttpTransportInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient __x_ABI_CWindows_CWeb_CHttp_CIHttpClient;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2 __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3 __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpContent __x_ABI_CWindows_CWeb_CHttp_CIHttpContent;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2 __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationProgressHandler_2_HSTRING_UINT64 __FIAsyncOperationProgressHandler_2_HSTRING_UINT64;

typedef interface __FIAsyncOperationWithProgress_2_HSTRING_UINT64 __FIAsyncOperationWithProgress_2_HSTRING_UINT64;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64 __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64* This,
        __FIAsyncOperationWithProgress_2_HSTRING_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_HSTRING_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_HSTRING_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_HSTRING_UINT64 __FIAsyncOperationWithProgress_2_HSTRING_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_HSTRING_UINT64;

typedef struct __FIAsyncOperationWithProgress_2_HSTRING_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        __FIAsyncOperationProgressHandler_2_HSTRING_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        __FIAsyncOperationProgressHandler_2_HSTRING_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_HSTRING_UINT64* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_HSTRING_UINT64Vtbl;

interface __FIAsyncOperationWithProgress_2_HSTRING_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_HSTRING_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_HSTRING_UINT64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_HSTRING_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_HSTRING_UINT64 __FIAsyncOperationProgressHandler_2_HSTRING_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_HSTRING_UINT64;

typedef struct __FIAsyncOperationProgressHandler_2_HSTRING_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_HSTRING_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_HSTRING_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_HSTRING_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_HSTRING_UINT64* This,
        __FIAsyncOperationWithProgress_2_HSTRING_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_HSTRING_UINT64Vtbl;

interface __FIAsyncOperationProgressHandler_2_HSTRING_UINT64
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_HSTRING_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_HSTRING_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_HSTRING_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_HSTRING_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_HSTRING_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_HSTRING_UINT64_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress __x_ABI_CWindows_CWeb_CHttp_CHttpProgress;

typedef interface __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64 __FIAsyncOperationProgressHandler_2_UINT64_UINT64;

typedef interface __FIAsyncOperationWithProgress_2_UINT64_UINT64 __FIAsyncOperationWithProgress_2_UINT64_UINT64;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64 __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_UINT64_UINT64 __FIAsyncOperationWithProgress_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_UINT64_UINT64;

typedef struct __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationProgressHandler_2_UINT64_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationProgressHandler_2_UINT64_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT64_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_UINT64_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationWithProgress_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_UINT64_UINT64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_UINT64_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64 __FIAsyncOperationProgressHandler_2_UINT64_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_UINT64_UINT64;

typedef struct __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_UINT64_UINT64* This,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl;

interface __FIAsyncOperationProgressHandler_2_UINT64_UINT64
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_UINT64_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_UINT64_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_UINT64_UINT64_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64 __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64 __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64 __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64 __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_UINT64_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress* asyncInfo,
        struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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
#if !defined(____FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CHttp__CHttpCookie;

typedef struct __FIIterator_1_Windows__CWeb__CHttp__CHttpCookieVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CHttp__CHttpCookieVtbl;

interface __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CHttp__CHttpCookieVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CHttp__CHttpCookie;

typedef struct __FIIterable_1_Windows__CWeb__CHttp__CHttpCookieVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __FIIterator_1_Windows__CWeb__CHttp__CHttpCookie** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CHttp__CHttpCookieVtbl;

interface __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CHttp__CHttpCookieVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CHttp__CIHttpContent;

typedef struct __FIIterator_1_Windows__CWeb__CHttp__CIHttpContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CHttp__CIHttpContent* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CHttp__CIHttpContentVtbl;

interface __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CHttp__CIHttpContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CHttp__CIHttpContent;

typedef struct __FIIterable_1_Windows__CWeb__CHttp__CIHttpContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CHttp__CIHttpContent* This,
        __FIIterator_1_Windows__CWeb__CHttp__CIHttpContent** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CHttp__CIHttpContentVtbl;

interface __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CHttp__CIHttpContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CHttp__CIHttpContent_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_IInspectable __FIMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_IInspectable;

typedef struct __FIMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_IInspectable* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_IInspectable* This);

    END_INTERFACE
} __FIMap_2_HSTRING_IInspectableVtbl;

interface __FIMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_IInspectable_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_IInspectable_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_IInspectable_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_IInspectable_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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
#if !defined(____FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie;

typedef struct __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookieVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookieVtbl;

interface __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookieVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_UINT64_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT64 __FIReference_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT64;

typedef struct __FIReference_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIReference_1_UINT64Vtbl;

interface __FIReference_1_UINT64
{
    CONST_VTBL struct __FIReference_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT64_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIStringable __x_ABI_CWindows_CFoundation_CIStringable;

#endif // ____x_ABI_CWindows_CFoundation_CIStringable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding;

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpProgressStage __x_ABI_CWindows_CWeb_CHttp_CHttpProgressStage;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpResponseMessageSource __x_ABI_CWindows_CWeb_CHttp_CHttpResponseMessageSource;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode;

typedef enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion __x_ABI_CWindows_CWeb_CHttp_CHttpVersion;

/*
 *
 * Struct Windows.Web.Http.HttpCompletionOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption
{
    HttpCompletionOption_ResponseContentRead = 0,
    HttpCompletionOption_ResponseHeadersRead = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpProgressStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CHttpProgressStage
{
    HttpProgressStage_None = 0,
    HttpProgressStage_DetectingProxy = 10,
    HttpProgressStage_ResolvingName = 20,
    HttpProgressStage_ConnectingToServer = 30,
    HttpProgressStage_NegotiatingSsl = 40,
    HttpProgressStage_SendingHeaders = 50,
    HttpProgressStage_SendingContent = 60,
    HttpProgressStage_WaitingForResponse = 70,
    HttpProgressStage_ReceivingHeaders = 80,
    HttpProgressStage_ReceivingContent = 90,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpResponseMessageSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CHttpResponseMessageSource
{
    HttpResponseMessageSource_None = 0,
    HttpResponseMessageSource_Cache = 1,
    HttpResponseMessageSource_Network = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpStatusCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode
{
    HttpStatusCode_None = 0,
    HttpStatusCode_Continue = 100,
    HttpStatusCode_SwitchingProtocols = 101,
    HttpStatusCode_Processing = 102,
    HttpStatusCode_Ok = 200,
    HttpStatusCode_Created = 201,
    HttpStatusCode_Accepted = 202,
    HttpStatusCode_NonAuthoritativeInformation = 203,
    HttpStatusCode_NoContent = 204,
    HttpStatusCode_ResetContent = 205,
    HttpStatusCode_PartialContent = 206,
    HttpStatusCode_MultiStatus = 207,
    HttpStatusCode_AlreadyReported = 208,
    HttpStatusCode_IMUsed = 226,
    HttpStatusCode_MultipleChoices = 300,
    HttpStatusCode_MovedPermanently = 301,
    HttpStatusCode_Found = 302,
    HttpStatusCode_SeeOther = 303,
    HttpStatusCode_NotModified = 304,
    HttpStatusCode_UseProxy = 305,
    HttpStatusCode_TemporaryRedirect = 307,
    HttpStatusCode_PermanentRedirect = 308,
    HttpStatusCode_BadRequest = 400,
    HttpStatusCode_Unauthorized = 401,
    HttpStatusCode_PaymentRequired = 402,
    HttpStatusCode_Forbidden = 403,
    HttpStatusCode_NotFound = 404,
    HttpStatusCode_MethodNotAllowed = 405,
    HttpStatusCode_NotAcceptable = 406,
    HttpStatusCode_ProxyAuthenticationRequired = 407,
    HttpStatusCode_RequestTimeout = 408,
    HttpStatusCode_Conflict = 409,
    HttpStatusCode_Gone = 410,
    HttpStatusCode_LengthRequired = 411,
    HttpStatusCode_PreconditionFailed = 412,
    HttpStatusCode_RequestEntityTooLarge = 413,
    HttpStatusCode_RequestUriTooLong = 414,
    HttpStatusCode_UnsupportedMediaType = 415,
    HttpStatusCode_RequestedRangeNotSatisfiable = 416,
    HttpStatusCode_ExpectationFailed = 417,
    HttpStatusCode_UnprocessableEntity = 422,
    HttpStatusCode_Locked = 423,
    HttpStatusCode_FailedDependency = 424,
    HttpStatusCode_UpgradeRequired = 426,
    HttpStatusCode_PreconditionRequired = 428,
    HttpStatusCode_TooManyRequests = 429,
    HttpStatusCode_RequestHeaderFieldsTooLarge = 431,
    HttpStatusCode_InternalServerError = 500,
    HttpStatusCode_NotImplemented = 501,
    HttpStatusCode_BadGateway = 502,
    HttpStatusCode_ServiceUnavailable = 503,
    HttpStatusCode_GatewayTimeout = 504,
    HttpStatusCode_HttpVersionNotSupported = 505,
    HttpStatusCode_VariantAlsoNegotiates = 506,
    HttpStatusCode_InsufficientStorage = 507,
    HttpStatusCode_LoopDetected = 508,
    HttpStatusCode_NotExtended = 510,
    HttpStatusCode_NetworkAuthenticationRequired = 511,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpVersion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion
{
    HttpVersion_None = 0,
    HttpVersion_Http10 = 1,
    HttpVersion_Http11 = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HttpVersion_Http20 = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Web.Http.HttpProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CWeb_CHttp_CHttpProgress
{
    enum __x_ABI_CWindows_CWeb_CHttp_CHttpProgressStage Stage;
    UINT64 BytesSent;
    __FIReference_1_UINT64* TotalBytesToSend;
    UINT64 BytesReceived;
    __FIReference_1_UINT64* TotalBytesToReceive;
    UINT32 Retries;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpBufferContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpBufferContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpBufferContentFactory[] = L"Windows.Web.Http.IHttpBufferContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromBuffer)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* content,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromBufferWithOffset)(__x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* content,
        UINT32 offset,
        UINT32 count,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_CreateFromBuffer(This, content, value) \
    ((This)->lpVtbl->CreateFromBuffer(This, content, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_CreateFromBufferWithOffset(This, content, offset, count, value) \
    ((This)->lpVtbl->CreateFromBufferWithOffset(This, content, offset, count, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpBufferContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient[] = L"Windows.Web.Http.IHttpClient";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* GetAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* GetWithOptionAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption completionOption,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* GetBufferAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* GetInputStreamAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* GetStringAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_HSTRING_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* PostAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* PutAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* SendRequestAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* SendRequestWithOptionAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption completionOption,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpResponseMessage_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* get_DefaultRequestHeaders)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient* This,
        __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpClientVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_DeleteAsync(This, uri, operation) \
    ((This)->lpVtbl->DeleteAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetAsync(This, uri, operation) \
    ((This)->lpVtbl->GetAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetWithOptionAsync(This, uri, completionOption, operation) \
    ((This)->lpVtbl->GetWithOptionAsync(This, uri, completionOption, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetBufferAsync(This, uri, operation) \
    ((This)->lpVtbl->GetBufferAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetInputStreamAsync(This, uri, operation) \
    ((This)->lpVtbl->GetInputStreamAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_GetStringAsync(This, uri, operation) \
    ((This)->lpVtbl->GetStringAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_PostAsync(This, uri, content, operation) \
    ((This)->lpVtbl->PostAsync(This, uri, content, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_PutAsync(This, uri, content, operation) \
    ((This)->lpVtbl->PutAsync(This, uri, content, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_SendRequestAsync(This, request, operation) \
    ((This)->lpVtbl->SendRequestAsync(This, request, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_SendRequestWithOptionAsync(This, request, completionOption, operation) \
    ((This)->lpVtbl->SendRequestWithOptionAsync(This, request, completionOption, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient_get_DefaultRequestHeaders(This, value) \
    ((This)->lpVtbl->get_DefaultRequestHeaders(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpClient2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient2[] = L"Windows.Web.Http.IHttpClient2";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryDeleteAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryGetAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryGetAsync2)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption completionOption,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryGetBufferAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetBufferResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryGetInputStreamAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetInputStreamResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryGetStringAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpGetStringResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryPostAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TryPutAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TrySendRequestAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);
    HRESULT (STDMETHODCALLTYPE* TrySendRequestAsync2)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient2* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* request,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpCompletionOption completionOption,
        __FIAsyncOperationWithProgress_2_Windows__CWeb__CHttp__CHttpRequestResult_Windows__CWeb__CHttp__CHttpProgress** operation);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryDeleteAsync(This, uri, operation) \
    ((This)->lpVtbl->TryDeleteAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryGetAsync(This, uri, operation) \
    ((This)->lpVtbl->TryGetAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryGetAsync2(This, uri, completionOption, operation) \
    ((This)->lpVtbl->TryGetAsync2(This, uri, completionOption, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryGetBufferAsync(This, uri, operation) \
    ((This)->lpVtbl->TryGetBufferAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryGetInputStreamAsync(This, uri, operation) \
    ((This)->lpVtbl->TryGetInputStreamAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryGetStringAsync(This, uri, operation) \
    ((This)->lpVtbl->TryGetStringAsync(This, uri, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryPostAsync(This, uri, content, operation) \
    ((This)->lpVtbl->TryPostAsync(This, uri, content, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TryPutAsync(This, uri, content, operation) \
    ((This)->lpVtbl->TryPutAsync(This, uri, content, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TrySendRequestAsync(This, request, operation) \
    ((This)->lpVtbl->TrySendRequestAsync(This, request, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_TrySendRequestAsync2(This, request, completionOption, operation) \
    ((This)->lpVtbl->TrySendRequestAsync2(This, request, completionOption, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpClient3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClient3[] = L"Windows.Web.Http.IHttpClient3";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DefaultPrivacyAnnotation)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultPrivacyAnnotation)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClient3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_get_DefaultPrivacyAnnotation(This, value) \
    ((This)->lpVtbl->get_DefaultPrivacyAnnotation(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_put_DefaultPrivacyAnnotation(This, value) \
    ((This)->lpVtbl->put_DefaultPrivacyAnnotation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClient3;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClient3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Web.Http.IHttpClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpClientFactory[] = L"Windows.Web.Http.IHttpClientFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory* This,
        __x_ABI_CWindows_CWeb_CHttp_CFilters_CIHttpFilter* filter,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpClient** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_Create(This, filter, value) \
    ((This)->lpVtbl->Create(This, filter, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpContent[] = L"Windows.Web.Http.IHttpContent";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpContentHeaderCollection** value);
    HRESULT (STDMETHODCALLTYPE* BufferAllAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* ReadAsBufferAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIBuffer_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* ReadAsInputStreamAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __FIAsyncOperationWithProgress_2_Windows__CStorage__CStreams__CIInputStream_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* ReadAsStringAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __FIAsyncOperationWithProgress_2_HSTRING_UINT64** operation);
    HRESULT (STDMETHODCALLTYPE* TryComputeLength)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        UINT64* length,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* WriteToStreamAsync)(__x_ABI_CWindows_CWeb_CHttp_CIHttpContent* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* outputStream,
        __FIAsyncOperationWithProgress_2_UINT64_UINT64** operation);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpContentVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpContent
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_BufferAllAsync(This, operation) \
    ((This)->lpVtbl->BufferAllAsync(This, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_ReadAsBufferAsync(This, operation) \
    ((This)->lpVtbl->ReadAsBufferAsync(This, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_ReadAsInputStreamAsync(This, operation) \
    ((This)->lpVtbl->ReadAsInputStreamAsync(This, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_ReadAsStringAsync(This, operation) \
    ((This)->lpVtbl->ReadAsStringAsync(This, operation))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_TryComputeLength(This, length, succeeded) \
    ((This)->lpVtbl->TryComputeLength(This, length, succeeded))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpContent_WriteToStreamAsync(This, outputStream, operation) \
    ((This)->lpVtbl->WriteToStreamAsync(This, outputStream, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookie
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookie
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookie[] = L"Windows.Web.Http.IHttpCookie";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Domain)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Path)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Expires)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Expires)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_HttpOnly)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_HttpOnly)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Secure)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Secure)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Domain(This, value) \
    ((This)->lpVtbl->get_Domain(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Path(This, value) \
    ((This)->lpVtbl->get_Path(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Expires(This, value) \
    ((This)->lpVtbl->get_Expires(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_put_Expires(This, value) \
    ((This)->lpVtbl->put_Expires(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_HttpOnly(This, value) \
    ((This)->lpVtbl->get_HttpOnly(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_put_HttpOnly(This, value) \
    ((This)->lpVtbl->put_HttpOnly(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Secure(This, value) \
    ((This)->lpVtbl->get_Secure(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_put_Secure(This, value) \
    ((This)->lpVtbl->put_Secure(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookie;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookieFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookie
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookieFactory[] = L"Windows.Web.Http.IHttpCookieFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory* This,
        HSTRING name,
        HSTRING domain,
        HSTRING path,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_Create(This, name, domain, path, value) \
    ((This)->lpVtbl->Create(This, name, domain, path, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpCookieManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpCookieManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpCookieManager[] = L"Windows.Web.Http.IHttpCookieManager";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetCookie)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* cookie,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetCookieWithThirdParty)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* cookie,
        boolean thirdParty,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* DeleteCookie)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* cookie);
    HRESULT (STDMETHODCALLTYPE* GetCookies)(__x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie** result);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManagerVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_SetCookie(This, cookie, result) \
    ((This)->lpVtbl->SetCookie(This, cookie, result))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_SetCookieWithThirdParty(This, cookie, thirdParty, result) \
    ((This)->lpVtbl->SetCookieWithThirdParty(This, cookie, thirdParty, result))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_DeleteCookie(This, cookie) \
    ((This)->lpVtbl->DeleteCookie(This, cookie))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_GetCookies(This, uri, result) \
    ((This)->lpVtbl->GetCookies(This, uri, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpCookieManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpFormUrlEncodedContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpFormUrlEncodedContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpFormUrlEncodedContentFactory[] = L"Windows.Web.Http.IHttpFormUrlEncodedContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* content,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_Create(This, content, value) \
    ((This)->lpVtbl->Create(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpFormUrlEncodedContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpGetBufferResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetBufferResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetBufferResult[] = L"Windows.Web.Http.IHttpGetBufferResult";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResultVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_get_ResponseMessage(This, value) \
    ((This)->lpVtbl->get_ResponseMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetBufferResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpGetInputStreamResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetInputStreamResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetInputStreamResult[] = L"Windows.Web.Http.IHttpGetInputStreamResult";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResultVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_get_ResponseMessage(This, value) \
    ((This)->lpVtbl->get_ResponseMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetInputStreamResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpGetStringResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpGetStringResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpGetStringResult[] = L"Windows.Web.Http.IHttpGetStringResult";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResultVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_get_ResponseMessage(This, value) \
    ((This)->lpVtbl->get_ResponseMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpGetStringResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethod[] = L"Windows.Web.Http.IHttpMethod";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Method)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_get_Method(This, value) \
    ((This)->lpVtbl->get_Method(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethod;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethod_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMethodFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethodFactory[] = L"Windows.Web.Http.IHttpMethodFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory* This,
        HSTRING method,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_Create(This, method, value) \
    ((This)->lpVtbl->Create(This, method, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMethodStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMethod
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMethodStatics[] = L"Windows.Web.Http.IHttpMethodStatics";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Delete)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Get)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Head)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Patch)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Post)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* get_Put)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStaticsVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Delete(This, value) \
    ((This)->lpVtbl->get_Delete(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Get(This, value) \
    ((This)->lpVtbl->get_Get(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Head(This, value) \
    ((This)->lpVtbl->get_Head(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Patch(This, value) \
    ((This)->lpVtbl->get_Patch(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Post(This, value) \
    ((This)->lpVtbl->get_Post(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_get_Put(This, value) \
    ((This)->lpVtbl->get_Put(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMethodStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartContent[] = L"Windows.Web.Http.IHttpMultipartContent";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Add)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_Add(This, content) \
    ((This)->lpVtbl->Add(This, content))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartContentFactory[] = L"Windows.Web.Http.IHttpMultipartContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithSubtype)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        HSTRING subtype,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithSubtypeAndBoundary)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory* This,
        HSTRING subtype,
        HSTRING boundary,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_CreateWithSubtype(This, subtype, value) \
    ((This)->lpVtbl->CreateWithSubtype(This, subtype, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_CreateWithSubtypeAndBoundary(This, subtype, boundary, value) \
    ((This)->lpVtbl->CreateWithSubtypeAndBoundary(This, subtype, boundary, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartFormDataContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartFormDataContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartFormDataContent[] = L"Windows.Web.Http.IHttpMultipartFormDataContent";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Add)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content);
    HRESULT (STDMETHODCALLTYPE* AddWithName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        HSTRING name);
    HRESULT (STDMETHODCALLTYPE* AddWithNameAndFileName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* content,
        HSTRING name,
        HSTRING fileName);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_Add(This, content) \
    ((This)->lpVtbl->Add(This, content))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_AddWithName(This, content, name) \
    ((This)->lpVtbl->AddWithName(This, content, name))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_AddWithNameAndFileName(This, content, name, fileName) \
    ((This)->lpVtbl->AddWithNameAndFileName(This, content, name, fileName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpMultipartFormDataContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpMultipartFormDataContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpMultipartFormDataContentFactory[] = L"Windows.Web.Http.IHttpMultipartFormDataContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithBoundary)(__x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory* This,
        HSTRING boundary,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_CreateWithBoundary(This, boundary, value) \
    ((This)->lpVtbl->CreateWithBoundary(This, boundary, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpMultipartFormDataContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessage[] = L"Windows.Web.Http.IHttpRequestMessage";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* value);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpRequestHeaderCollection** value);
    HRESULT (STDMETHODCALLTYPE* get_Method)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod** value);
    HRESULT (STDMETHODCALLTYPE* put_Method)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __FIMap_2_HSTRING_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_RequestUri)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_RequestUri)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportInformation)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_Method(This, value) \
    ((This)->lpVtbl->get_Method(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_put_Method(This, value) \
    ((This)->lpVtbl->put_Method(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_RequestUri(This, value) \
    ((This)->lpVtbl->get_RequestUri(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_put_RequestUri(This, value) \
    ((This)->lpVtbl->put_RequestUri(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_get_TransportInformation(This, value) \
    ((This)->lpVtbl->get_TransportInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessage2[] = L"Windows.Web.Http.IHttpRequestMessage2";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrivacyAnnotation)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PrivacyAnnotation)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2Vtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_get_PrivacyAnnotation(This, value) \
    ((This)->lpVtbl->get_PrivacyAnnotation(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_put_PrivacyAnnotation(This, value) \
    ((This)->lpVtbl->put_PrivacyAnnotation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestMessageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestMessageFactory[] = L"Windows.Web.Http.IHttpRequestMessageFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpMethod* method,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_Create(This, method, uri, value) \
    ((This)->lpVtbl->Create(This, method, uri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpRequestResult[] = L"Windows.Web.Http.IHttpRequestResult";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResultVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_get_ResponseMessage(This, value) \
    ((This)->lpVtbl->get_ResponseMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Web.Http.IHttpResponseMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpResponseMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpResponseMessage[] = L"Windows.Web.Http.IHttpResponseMessage";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent* value);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CHeaders_CIHttpResponseHeaderCollection** value);
    HRESULT (STDMETHODCALLTYPE* get_IsSuccessStatusCode)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ReasonPhrase)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ReasonPhrase)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* put_RequestMessage)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpResponseMessageSource* value);
    HRESULT (STDMETHODCALLTYPE* put_Source)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpResponseMessageSource value);
    HRESULT (STDMETHODCALLTYPE* get_StatusCode)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode* value);
    HRESULT (STDMETHODCALLTYPE* put_StatusCode)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion* value);
    HRESULT (STDMETHODCALLTYPE* put_Version)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpVersion value);
    HRESULT (STDMETHODCALLTYPE* EnsureSuccessStatusCode)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** result);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_IsSuccessStatusCode(This, value) \
    ((This)->lpVtbl->get_IsSuccessStatusCode(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_ReasonPhrase(This, value) \
    ((This)->lpVtbl->get_ReasonPhrase(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_ReasonPhrase(This, value) \
    ((This)->lpVtbl->put_ReasonPhrase(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_RequestMessage(This, value) \
    ((This)->lpVtbl->get_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_RequestMessage(This, value) \
    ((This)->lpVtbl->put_RequestMessage(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_Source(This, value) \
    ((This)->lpVtbl->put_Source(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_StatusCode(This, value) \
    ((This)->lpVtbl->get_StatusCode(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_StatusCode(This, value) \
    ((This)->lpVtbl->put_StatusCode(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_put_Version(This, value) \
    ((This)->lpVtbl->put_Version(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_EnsureSuccessStatusCode(This, result) \
    ((This)->lpVtbl->EnsureSuccessStatusCode(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpResponseMessageFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpResponseMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpResponseMessageFactory[] = L"Windows.Web.Http.IHttpResponseMessageFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory* This,
        enum __x_ABI_CWindows_CWeb_CHttp_CHttpStatusCode statusCode,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_Create(This, statusCode, value) \
    ((This)->lpVtbl->Create(This, statusCode, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessageFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpStreamContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpStreamContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpStreamContentFactory[] = L"Windows.Web.Http.IHttpStreamContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromInputStream)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* content,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_CreateFromInputStream(This, content, value) \
    ((This)->lpVtbl->CreateFromInputStream(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStreamContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpStringContentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpStringContent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpStringContentFactory[] = L"Windows.Web.Http.IHttpStringContentFactory";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromString)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        HSTRING content,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromStringWithEncoding)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        HSTRING content,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromStringWithEncodingAndMediaType)(__x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory* This,
        HSTRING content,
        enum __x_ABI_CWindows_CStorage_CStreams_CUnicodeEncoding encoding,
        HSTRING mediaType,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpContent** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactoryVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_CreateFromString(This, content, value) \
    ((This)->lpVtbl->CreateFromString(This, content, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_CreateFromStringWithEncoding(This, content, encoding, value) \
    ((This)->lpVtbl->CreateFromStringWithEncoding(This, content, encoding, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_CreateFromStringWithEncodingAndMediaType(This, content, encoding, mediaType, value) \
    ((This)->lpVtbl->CreateFromStringWithEncodingAndMediaType(This, content, encoding, mediaType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpStringContentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.Http.IHttpTransportInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.Http.HttpTransportInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_Http_IHttpTransportInformation[] = L"Windows.Web.Http.IHttpTransportInformation";
typedef struct __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificate)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrorSeverity)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        enum __x_ABI_CWindows_CNetworking_CSockets_CSocketSslErrorSeverity* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerCertificateErrors)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CChainValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* get_ServerIntermediateCertificates)(__x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformationVtbl;

interface __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_get_ServerCertificate(This, value) \
    ((This)->lpVtbl->get_ServerCertificate(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_get_ServerCertificateErrorSeverity(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrorSeverity(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_get_ServerCertificateErrors(This, value) \
    ((This)->lpVtbl->get_ServerCertificateErrors(This, value))

#define __x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_get_ServerIntermediateCertificates(This, value) \
    ((This)->lpVtbl->get_ServerIntermediateCertificates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation;
#endif /* !defined(____x_ABI_CWindows_CWeb_CHttp_CIHttpTransportInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpBufferContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpBufferContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpBufferContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpBufferContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpBufferContent[] = L"Windows.Web.Http.HttpBufferContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpClient ** Default Interface **
 *    Windows.Web.Http.IHttpClient2
 *    Windows.Web.Http.IHttpClient3
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpClient_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpClient_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpClient[] = L"Windows.Web.Http.HttpClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookie
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpCookieFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpCookie ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookie_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookie_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookie[] = L"Windows.Web.Http.HttpCookie";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookieCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Web.Http.HttpCookie> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.HttpCookie>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookieCollection_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookieCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookieCollection[] = L"Windows.Web.Http.HttpCookieCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpCookieManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpCookieManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpCookieManager_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpCookieManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpCookieManager[] = L"Windows.Web.Http.HttpCookieManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpFormUrlEncodedContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpFormUrlEncodedContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpFormUrlEncodedContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpFormUrlEncodedContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpFormUrlEncodedContent[] = L"Windows.Web.Http.HttpFormUrlEncodedContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpGetBufferResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetBufferResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetBufferResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetBufferResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetBufferResult[] = L"Windows.Web.Http.HttpGetBufferResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpGetInputStreamResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetInputStreamResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetInputStreamResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetInputStreamResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetInputStreamResult[] = L"Windows.Web.Http.HttpGetInputStreamResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpGetStringResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpGetStringResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpGetStringResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpGetStringResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpGetStringResult[] = L"Windows.Web.Http.HttpGetStringResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpMethodFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.Http.IHttpMethodStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMethod ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMethod_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMethod_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMethod[] = L"Windows.Web.Http.HttpMethod";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpMultipartContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpMultipartContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMultipartContent
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.IHttpContent>
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMultipartContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMultipartContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMultipartContent[] = L"Windows.Web.Http.HttpMultipartContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpMultipartFormDataContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpMultipartFormDataContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpMultipartFormDataContent
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.Collections.IIterable`1<Windows.Web.Http.IHttpContent>
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpMultipartFormDataContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpMultipartFormDataContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpMultipartFormDataContent[] = L"Windows.Web.Http.HttpMultipartFormDataContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpRequestMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpRequestMessageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpRequestMessage ** Default Interface **
 *    Windows.Web.Http.IHttpRequestMessage2
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpRequestMessage_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpRequestMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpRequestMessage[] = L"Windows.Web.Http.HttpRequestMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpRequestResult ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpRequestResult[] = L"Windows.Web.Http.HttpRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Web.Http.HttpResponseMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Web.Http.IHttpResponseMessageFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpResponseMessage ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpResponseMessage_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpResponseMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpResponseMessage[] = L"Windows.Web.Http.HttpResponseMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpStreamContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpStreamContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpStreamContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpStreamContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpStreamContent[] = L"Windows.Web.Http.HttpStreamContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpStringContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Web.Http.IHttpStringContentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpContent ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Foundation.IStringable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpStringContent_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpStringContent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpStringContent[] = L"Windows.Web.Http.HttpStringContent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.Http.HttpTransportInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.Http.IHttpTransportInformation ** Default Interface **
 *    Windows.Foundation.IStringable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_Http_HttpTransportInformation_DEFINED
#define RUNTIMECLASS_Windows_Web_Http_HttpTransportInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_Http_HttpTransportInformation[] = L"Windows.Web.Http.HttpTransportInformation";
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
#endif // __windows2Eweb2Ehttp_p_h__

#endif // __windows2Eweb2Ehttp_h__
