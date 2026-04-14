
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
#ifndef __windows2Esecurity2Eauthentication2Eweb_h__
#define __windows2Esecurity2Eauthentication2Eweb_h__
#ifndef __windows2Esecurity2Eauthentication2Eweb_p_h__
#define __windows2Esecurity2Eauthentication2Eweb_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    interface IWebAuthenticationBrokerStatics;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics ABI::Windows::Security::Authentication::Web::IWebAuthenticationBrokerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    interface IWebAuthenticationBrokerStatics2;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2 ABI::Windows::Security::Authentication::Web::IWebAuthenticationBrokerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    interface IWebAuthenticationResult;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult ABI::Windows::Security::Authentication::Web::IWebAuthenticationResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    class WebAuthenticationResult;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b34952ac-265e-5947-8735-e9318f4301ff"))
IAsyncOperation<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*, ABI::Windows::Security::Authentication::Web::IWebAuthenticationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Web.WebAuthenticationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c1ec44c-e942-54e5-bcd3-e329c951f595"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*, ABI::Windows::Security::Authentication::Web::IWebAuthenticationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Web.WebAuthenticationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::WebAuthenticationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace Authentication {
                namespace Web {
                    typedef enum WebAuthenticationOptions : unsigned int WebAuthenticationOptions;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    typedef enum WebAuthenticationStatus : int WebAuthenticationStatus;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Authentication.Web.TokenBindingKeyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    enum TokenBindingKeyType : int
                    {
                        TokenBindingKeyType_Rsa2048 = 0,
                        TokenBindingKeyType_EcdsaP256 = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        TokenBindingKeyType_AnyExisting = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    };
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.WebAuthenticationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    enum WebAuthenticationOptions : unsigned int
                    {
                        WebAuthenticationOptions_None = 0,
                        WebAuthenticationOptions_SilentMode = 0x1,
                        WebAuthenticationOptions_UseTitle = 0x2,
                        WebAuthenticationOptions_UseHttpPost = 0x4,
                        WebAuthenticationOptions_UseCorporateNetwork = 0x8,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(WebAuthenticationOptions)
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.WebAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    enum WebAuthenticationStatus : int
                    {
                        WebAuthenticationStatus_Success = 0,
                        WebAuthenticationStatus_UserCancel = 1,
                        WebAuthenticationStatus_ErrorHttp = 2,
                    };
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationBrokerStatics[] = L"Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    MIDL_INTERFACE("2f149f1a-e673-40b5-bc22-201a6864a37b")
                    IWebAuthenticationBrokerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateWithCallbackUriAsync(
                            ABI::Windows::Security::Authentication::Web::WebAuthenticationOptions options,
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            ABI::Windows::Foundation::IUriRuntimeClass* callbackUri,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateWithoutCallbackUriAsync(
                            ABI::Windows::Security::Authentication::Web::WebAuthenticationOptions options,
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentApplicationCallbackUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** callbackUri
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWebAuthenticationBrokerStatics = __uuidof(IWebAuthenticationBrokerStatics);
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationBrokerStatics2[] = L"Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    MIDL_INTERFACE("73cdfb9e-14e7-41da-a971-aaf4410b621e")
                    IWebAuthenticationBrokerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateAndContinue(
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateWithCallbackUriAndContinue(
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            ABI::Windows::Foundation::IUriRuntimeClass* callbackUri
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            ABI::Windows::Foundation::IUriRuntimeClass* callbackUri,
                            ABI::Windows::Foundation::Collections::IPropertySet* continuationData,
                            ABI::Windows::Security::Authentication::Web::WebAuthenticationOptions options
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateSilentlyAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateSilentlyWithOptionsAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* requestUri,
                            ABI::Windows::Security::Authentication::Web::WebAuthenticationOptions options,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWebAuthenticationBrokerStatics2 = __uuidof(IWebAuthenticationBrokerStatics2);
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationResult[] = L"Windows.Security.Authentication.Web.IWebAuthenticationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    MIDL_INTERFACE("64002b4b-ede9-470a-a5cd-0323faf6e262")
                    IWebAuthenticationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseData(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseStatus(
                            ABI::Windows::Security::Authentication::Web::WebAuthenticationStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseErrorDetail(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWebAuthenticationResult = __uuidof(IWebAuthenticationResult);
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationBroker_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationBroker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_WebAuthenticationBroker[] = L"Windows.Security.Authentication.Web.WebAuthenticationBroker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.WebAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.IWebAuthenticationResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_WebAuthenticationResult[] = L"Windows.Security.Authentication.Web.WebAuthenticationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationStatus __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationStatus;

/*
 *
 * Struct Windows.Security.Authentication.Web.TokenBindingKeyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CTokenBindingKeyType
{
    TokenBindingKeyType_Rsa2048 = 0,
    TokenBindingKeyType_EcdsaP256 = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    TokenBindingKeyType_AnyExisting = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.WebAuthenticationOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions
{
    WebAuthenticationOptions_None = 0,
    WebAuthenticationOptions_SilentMode = 0x1,
    WebAuthenticationOptions_UseTitle = 0x2,
    WebAuthenticationOptions_UseHttpPost = 0x4,
    WebAuthenticationOptions_UseCorporateNetwork = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.WebAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationStatus
{
    WebAuthenticationStatus_Success = 0,
    WebAuthenticationStatus_UserCancel = 1,
    WebAuthenticationStatus_ErrorHttp = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationBrokerStatics[] = L"Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AuthenticateWithCallbackUriAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions options,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* callbackUri,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* AuthenticateWithoutCallbackUriAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions options,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetCurrentApplicationCallbackUri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** callbackUri);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_AuthenticateWithCallbackUriAsync(This, options, requestUri, callbackUri, asyncInfo) \
    ((This)->lpVtbl->AuthenticateWithCallbackUriAsync(This, options, requestUri, callbackUri, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_AuthenticateWithoutCallbackUriAsync(This, options, requestUri, asyncInfo) \
    ((This)->lpVtbl->AuthenticateWithoutCallbackUriAsync(This, options, requestUri, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_GetCurrentApplicationCallbackUri(This, callbackUri) \
    ((This)->lpVtbl->GetCurrentApplicationCallbackUri(This, callbackUri))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationBrokerStatics2[] = L"Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AuthenticateAndContinue)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri);
    HRESULT (STDMETHODCALLTYPE* AuthenticateWithCallbackUriAndContinue)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* callbackUri);
    HRESULT (STDMETHODCALLTYPE* AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* callbackUri,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* continuationData,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions options);
    HRESULT (STDMETHODCALLTYPE* AuthenticateSilentlyAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* AuthenticateSilentlyWithOptionsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* requestUri,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationOptions options,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CWebAuthenticationResult** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AuthenticateAndContinue(This, requestUri) \
    ((This)->lpVtbl->AuthenticateAndContinue(This, requestUri))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AuthenticateWithCallbackUriAndContinue(This, requestUri, callbackUri) \
    ((This)->lpVtbl->AuthenticateWithCallbackUriAndContinue(This, requestUri, callbackUri))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(This, requestUri, callbackUri, continuationData, options) \
    ((This)->lpVtbl->AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(This, requestUri, callbackUri, continuationData, options))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AuthenticateSilentlyAsync(This, requestUri, asyncInfo) \
    ((This)->lpVtbl->AuthenticateSilentlyAsync(This, requestUri, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_AuthenticateSilentlyWithOptionsAsync(This, requestUri, options, asyncInfo) \
    ((This)->lpVtbl->AuthenticateSilentlyWithOptionsAsync(This, requestUri, options, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationBrokerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.IWebAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.WebAuthenticationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_IWebAuthenticationResult[] = L"Windows.Security.Authentication.Web.IWebAuthenticationResult";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResponseData)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseStatus)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CWebAuthenticationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseErrorDetail)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_get_ResponseData(This, value) \
    ((This)->lpVtbl->get_ResponseData(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_get_ResponseStatus(This, value) \
    ((This)->lpVtbl->get_ResponseStatus(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_get_ResponseErrorDetail(This, value) \
    ((This)->lpVtbl->get_ResponseErrorDetail(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CIWebAuthenticationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.WebAuthenticationBroker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.IWebAuthenticationBrokerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationBroker_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationBroker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_WebAuthenticationBroker[] = L"Windows.Security.Authentication.Web.WebAuthenticationBroker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.WebAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.IWebAuthenticationResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_WebAuthenticationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_WebAuthenticationResult[] = L"Windows.Security.Authentication.Web.WebAuthenticationResult";
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
#endif // __windows2Esecurity2Eauthentication2Eweb_p_h__

#endif // __windows2Esecurity2Eauthentication2Eweb_h__
