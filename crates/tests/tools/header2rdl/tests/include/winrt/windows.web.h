
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
#ifndef __windows2Eweb_h__
#define __windows2Eweb_h__
#ifndef __windows2Eweb_p_h__
#define __windows2Eweb_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            interface IUriToStreamResolver;
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver ABI::Windows::Web::IUriToStreamResolver

#endif // ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            interface IWebErrorStatics;
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CIWebErrorStatics ABI::Windows::Web::IWebErrorStatics

#endif // ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8fe0732-556d-5841-b7ee-b3450fb52666"))
IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d0bd0125-9049-57a3-bd66-e2525d98c814"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Web {
            typedef enum WebErrorStatus : int WebErrorStatus;
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.WebErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Web {
            enum WebErrorStatus : int
            {
                WebErrorStatus_Unknown = 0,
                WebErrorStatus_CertificateCommonNameIsIncorrect = 1,
                WebErrorStatus_CertificateExpired = 2,
                WebErrorStatus_CertificateContainsErrors = 3,
                WebErrorStatus_CertificateRevoked = 4,
                WebErrorStatus_CertificateIsInvalid = 5,
                WebErrorStatus_ServerUnreachable = 6,
                WebErrorStatus_Timeout = 7,
                WebErrorStatus_ErrorHttpInvalidServerResponse = 8,
                WebErrorStatus_ConnectionAborted = 9,
                WebErrorStatus_ConnectionReset = 10,
                WebErrorStatus_Disconnected = 11,
                WebErrorStatus_HttpToHttpsOnRedirection = 12,
                WebErrorStatus_HttpsToHttpOnRedirection = 13,
                WebErrorStatus_CannotConnect = 14,
                WebErrorStatus_HostNameNotResolved = 15,
                WebErrorStatus_OperationCanceled = 16,
                WebErrorStatus_RedirectFailed = 17,
                WebErrorStatus_UnexpectedStatusCode = 18,
                WebErrorStatus_UnexpectedRedirection = 19,
                WebErrorStatus_UnexpectedClientError = 20,
                WebErrorStatus_UnexpectedServerError = 21,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                WebErrorStatus_InsufficientRangeSupport = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                WebErrorStatus_MissingContentLengthSupport = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                WebErrorStatus_MultipleChoices = 300,
                WebErrorStatus_MovedPermanently = 301,
                WebErrorStatus_Found = 302,
                WebErrorStatus_SeeOther = 303,
                WebErrorStatus_NotModified = 304,
                WebErrorStatus_UseProxy = 305,
                WebErrorStatus_TemporaryRedirect = 307,
                WebErrorStatus_BadRequest = 400,
                WebErrorStatus_Unauthorized = 401,
                WebErrorStatus_PaymentRequired = 402,
                WebErrorStatus_Forbidden = 403,
                WebErrorStatus_NotFound = 404,
                WebErrorStatus_MethodNotAllowed = 405,
                WebErrorStatus_NotAcceptable = 406,
                WebErrorStatus_ProxyAuthenticationRequired = 407,
                WebErrorStatus_RequestTimeout = 408,
                WebErrorStatus_Conflict = 409,
                WebErrorStatus_Gone = 410,
                WebErrorStatus_LengthRequired = 411,
                WebErrorStatus_PreconditionFailed = 412,
                WebErrorStatus_RequestEntityTooLarge = 413,
                WebErrorStatus_RequestUriTooLong = 414,
                WebErrorStatus_UnsupportedMediaType = 415,
                WebErrorStatus_RequestedRangeNotSatisfiable = 416,
                WebErrorStatus_ExpectationFailed = 417,
                WebErrorStatus_InternalServerError = 500,
                WebErrorStatus_NotImplemented = 501,
                WebErrorStatus_BadGateway = 502,
                WebErrorStatus_ServiceUnavailable = 503,
                WebErrorStatus_GatewayTimeout = 504,
                WebErrorStatus_HttpVersionNotSupported = 505,
            };
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.IUriToStreamResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_IUriToStreamResolver[] = L"Windows.Web.IUriToStreamResolver";
namespace ABI {
    namespace Windows {
        namespace Web {
            MIDL_INTERFACE("b0aba86a-9aeb-4d3a-9590-003e3ca7e290")
            IUriToStreamResolver : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE UriToStreamAsync(
                    ABI::Windows::Foundation::IUriRuntimeClass* uri,
                    __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUriToStreamResolver = __uuidof(IUriToStreamResolver);
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CIUriToStreamResolver;
#endif /* !defined(____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.IWebErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.WebError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_IWebErrorStatics[] = L"Windows.Web.IWebErrorStatics";
namespace ABI {
    namespace Windows {
        namespace Web {
            MIDL_INTERFACE("fe616766-bf27-4064-87b7-6563bb11ce2e")
            IWebErrorStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetStatus(
                    INT32 hresult,
                    ABI::Windows::Web::WebErrorStatus* status
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IWebErrorStatics = __uuidof(IWebErrorStatics);
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CIWebErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.WebError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.IWebErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_WebError_DEFINED
#define RUNTIMECLASS_Windows_Web_WebError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_WebError[] = L"Windows.Web.WebError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CIUriToStreamResolver __x_ABI_CWindows_CWeb_CIUriToStreamResolver;

#endif // ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CIWebErrorStatics __x_ABI_CWindows_CWeb_CIWebErrorStatics;

#endif // ____x_ABI_CWindows_CWeb_CIWebErrorStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CWebErrorStatus __x_ABI_CWindows_CWeb_CWebErrorStatus;

/*
 *
 * Struct Windows.Web.WebErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CWeb_CWebErrorStatus
{
    WebErrorStatus_Unknown = 0,
    WebErrorStatus_CertificateCommonNameIsIncorrect = 1,
    WebErrorStatus_CertificateExpired = 2,
    WebErrorStatus_CertificateContainsErrors = 3,
    WebErrorStatus_CertificateRevoked = 4,
    WebErrorStatus_CertificateIsInvalid = 5,
    WebErrorStatus_ServerUnreachable = 6,
    WebErrorStatus_Timeout = 7,
    WebErrorStatus_ErrorHttpInvalidServerResponse = 8,
    WebErrorStatus_ConnectionAborted = 9,
    WebErrorStatus_ConnectionReset = 10,
    WebErrorStatus_Disconnected = 11,
    WebErrorStatus_HttpToHttpsOnRedirection = 12,
    WebErrorStatus_HttpsToHttpOnRedirection = 13,
    WebErrorStatus_CannotConnect = 14,
    WebErrorStatus_HostNameNotResolved = 15,
    WebErrorStatus_OperationCanceled = 16,
    WebErrorStatus_RedirectFailed = 17,
    WebErrorStatus_UnexpectedStatusCode = 18,
    WebErrorStatus_UnexpectedRedirection = 19,
    WebErrorStatus_UnexpectedClientError = 20,
    WebErrorStatus_UnexpectedServerError = 21,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    WebErrorStatus_InsufficientRangeSupport = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    WebErrorStatus_MissingContentLengthSupport = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    WebErrorStatus_MultipleChoices = 300,
    WebErrorStatus_MovedPermanently = 301,
    WebErrorStatus_Found = 302,
    WebErrorStatus_SeeOther = 303,
    WebErrorStatus_NotModified = 304,
    WebErrorStatus_UseProxy = 305,
    WebErrorStatus_TemporaryRedirect = 307,
    WebErrorStatus_BadRequest = 400,
    WebErrorStatus_Unauthorized = 401,
    WebErrorStatus_PaymentRequired = 402,
    WebErrorStatus_Forbidden = 403,
    WebErrorStatus_NotFound = 404,
    WebErrorStatus_MethodNotAllowed = 405,
    WebErrorStatus_NotAcceptable = 406,
    WebErrorStatus_ProxyAuthenticationRequired = 407,
    WebErrorStatus_RequestTimeout = 408,
    WebErrorStatus_Conflict = 409,
    WebErrorStatus_Gone = 410,
    WebErrorStatus_LengthRequired = 411,
    WebErrorStatus_PreconditionFailed = 412,
    WebErrorStatus_RequestEntityTooLarge = 413,
    WebErrorStatus_RequestUriTooLong = 414,
    WebErrorStatus_UnsupportedMediaType = 415,
    WebErrorStatus_RequestedRangeNotSatisfiable = 416,
    WebErrorStatus_ExpectationFailed = 417,
    WebErrorStatus_InternalServerError = 500,
    WebErrorStatus_NotImplemented = 501,
    WebErrorStatus_BadGateway = 502,
    WebErrorStatus_ServiceUnavailable = 503,
    WebErrorStatus_GatewayTimeout = 504,
    WebErrorStatus_HttpVersionNotSupported = 505,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.IUriToStreamResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_IUriToStreamResolver[] = L"Windows.Web.IUriToStreamResolver";
typedef struct __x_ABI_CWindows_CWeb_CIUriToStreamResolverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UriToStreamAsync)(__x_ABI_CWindows_CWeb_CIUriToStreamResolver* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CIUriToStreamResolverVtbl;

interface __x_ABI_CWindows_CWeb_CIUriToStreamResolver
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CIUriToStreamResolverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver_UriToStreamAsync(This, uri, operation) \
    ((This)->lpVtbl->UriToStreamAsync(This, uri, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CIUriToStreamResolver;
#endif /* !defined(____x_ABI_CWindows_CWeb_CIUriToStreamResolver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Web.IWebErrorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Web.WebError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_IWebErrorStatics[] = L"Windows.Web.IWebErrorStatics";
typedef struct __x_ABI_CWindows_CWeb_CIWebErrorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CWeb_CIWebErrorStatics* This,
        INT32 hresult,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CIWebErrorStaticsVtbl;

interface __x_ABI_CWindows_CWeb_CIWebErrorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CIWebErrorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CIWebErrorStatics_GetStatus(This, hresult, status) \
    ((This)->lpVtbl->GetStatus(This, hresult, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CIWebErrorStatics;
#endif /* !defined(____x_ABI_CWindows_CWeb_CIWebErrorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Web.WebError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Web.IWebErrorStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Web_WebError_DEFINED
#define RUNTIMECLASS_Windows_Web_WebError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_WebError[] = L"Windows.Web.WebError";
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
#endif // __windows2Eweb_p_h__

#endif // __windows2Eweb_h__
