
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
#ifndef __windows2Esecurity2Eauthentication2Eweb2Ecore_h__
#define __windows2Esecurity2Eauthentication2Eweb2Ecore_h__
#ifndef __windows2Esecurity2Eauthentication2Eweb2Ecore_p_h__
#define __windows2Esecurity2Eauthentication2Eweb2Ecore_p_h__


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
#include "Windows.Security.Credentials.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IFindAllAccountsResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult ABI::Windows::Security::Authentication::Web::Core::IFindAllAccountsResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAccountEventArgs;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs ABI::Windows::Security::Authentication::Web::Core::IWebAccountEventArgs

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAccountMonitor;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor ABI::Windows::Security::Authentication::Web::Core::IWebAccountMonitor

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAccountMonitor2;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2 ABI::Windows::Security::Authentication::Web::Core::IWebAccountMonitor2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationAddAccountResponse;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResponse

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationAddAccountResponseFactory;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResponseFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationAddAccountResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationCoreManagerStatics;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationCoreManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationCoreManagerStatics2;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2 ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationCoreManagerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationCoreManagerStatics3;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3 ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationCoreManagerStatics3

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationCoreManagerStatics4;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4 ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationCoreManagerStatics4

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationCoreManagerStatics5;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5 ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationCoreManagerStatics5

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationTransferTokenRequest;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationTransferTokenRequest

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebAuthenticationTransferTokenRequestFactory;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationTransferTokenRequestFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebProviderError;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError ABI::Windows::Security::Authentication::Web::Core::IWebProviderError

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebProviderErrorFactory;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory ABI::Windows::Security::Authentication::Web::Core::IWebProviderErrorFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequest;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequest2;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2 ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequest3;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3 ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest3

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequestFactory;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenRequestResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenResponse;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        interface IWebTokenResponseFactory;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponseFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class FindAllAccountsResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9affb572-58c3-5c6c-9397-2b7704aa35c3"))
IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*, ABI::Windows::Security::Authentication::Web::Core::IFindAllAccountsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Web.Core.FindAllAccountsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5de9ba5e-0ade-5344-9fe4-987f1d387ef7"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*, ABI::Windows::Security::Authentication::Web::Core::IFindAllAccountsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Web.Core.FindAllAccountsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::FindAllAccountsResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebAuthenticationAddAccountResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("63824a59-b4fa-5e89-ba3c-d44e532a29d8"))
IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*, ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fe15f0e6-6520-5b5e-ae26-b52a506b980d"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*, ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebTokenRequestResult;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0a815852-7c44-5674-b3d2-fa2e4c1e46c9"))
IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Web.Core.WebTokenRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("deb54b22-70f2-55ab-97c0-6cbdc5ddb6f0"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Web.Core.WebTokenRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount ABI::Windows::Security::Credentials::IWebAccount

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("acd76b54-297f-5a18-9143-20a309e2dfd3"))
IAsyncOperation<ABI::Windows::Security::Credentials::WebAccount*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccount*, ABI::Windows::Security::Credentials::IWebAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.WebAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::WebAccount*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4bd6f1e5-ca89-5240-8f3d-7f1b54ae90a7"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::WebAccount*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccount*, ABI::Windows::Security::Credentials::IWebAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.WebAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::WebAccount*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccountProvider;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProvider;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider ABI::Windows::Security::Credentials::IWebAccountProvider

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88c66009-12f7-58e2-8dbe-6efc620c85ba"))
IAsyncOperation<ABI::Windows::Security::Credentials::WebAccountProvider*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccountProvider*, ABI::Windows::Security::Credentials::IWebAccountProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.WebAccountProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::WebAccountProvider*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9477622b-1340-5574-81fc-5013581f57c9"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::WebAccountProvider*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccountProvider*, ABI::Windows::Security::Credentials::IWebAccountProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.WebAccountProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::WebAccountProvider*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebTokenResponse;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f080b0c9-a095-5b3a-a1dc-d17e7d2982c7"))
IIterator<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.Web.Core.WebTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7e5bb7ec-bbd7-5575-9a61-f5815fa22a0e"))
IIterable<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.Web.Core.WebTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfb82cca-aebc-567c-95d9-eba25c365faa"))
IIterator<ABI::Windows::Security::Credentials::WebAccount*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccount*, ABI::Windows::Security::Credentials::IWebAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Credentials.WebAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Credentials::WebAccount*> __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb15d439-a910-542a-89ed-7cfe67848a83"))
IIterable<ABI::Windows::Security::Credentials::WebAccount*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccount*, ABI::Windows::Security::Credentials::IWebAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Credentials.WebAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Credentials::WebAccount*> __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("199e065c-8195-55da-9c10-8aeaf9ac1062"))
IVectorView<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*, ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Web.Core.WebTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::Web::Core::WebTokenResponse*> __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e0798d3d-2b4a-589a-ab12-02dccc158afc"))
IVectorView<ABI::Windows::Security::Credentials::WebAccount*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::WebAccount*, ABI::Windows::Security::Credentials::IWebAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Credentials.WebAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Credentials::WebAccount*> __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebAccountMonitor;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c8cb498d-e0da-52a1-abf9-7198c7f5cb42"))
ITypedEventHandler<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, ABI::Windows::Security::Authentication::Web::Core::IWebAccountMonitor*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Security.Authentication.Web.Core.WebAccountMonitor, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, IInspectable*> __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        class WebAccountEventArgs;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fa704f04-87b6-516b-9596-cd7cc092169b"))
ITypedEventHandler<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, ABI::Windows::Security::Authentication::Web::Core::WebAccountEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, ABI::Windows::Security::Authentication::Web::Core::IWebAccountMonitor*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Core::WebAccountEventArgs*, ABI::Windows::Security::Authentication::Web::Core::IWebAccountEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Security.Authentication.Web.Core.WebAccountMonitor, Windows.Security.Authentication.Web.Core.WebAccountEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Security::Authentication::Web::Core::WebAccountMonitor*, ABI::Windows::Security::Authentication::Web::Core::WebAccountEventArgs*> __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        typedef enum FindAllWebAccountsStatus : int FindAllWebAccountsStatus;
                    } /* Core */
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
                    namespace Core {
                        typedef enum WebAuthenticationAddAccountStatus : int WebAuthenticationAddAccountStatus;
                    } /* Core */
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
                    namespace Core {
                        typedef enum WebTokenRequestPromptType : int WebTokenRequestPromptType;
                    } /* Core */
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
                    namespace Core {
                        typedef enum WebTokenRequestStatus : int WebTokenRequestStatus;
                    } /* Core */
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
                    namespace Core {
                        class WebAuthenticationAddAccountResponse;
                    } /* Core */
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
                    namespace Core {
                        class WebAuthenticationTransferTokenRequest;
                    } /* Core */
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
                    namespace Core {
                        class WebProviderError;
                    } /* Core */
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
                    namespace Core {
                        class WebTokenRequest;
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        enum FindAllWebAccountsStatus : int
                        {
                            FindAllWebAccountsStatus_Success = 0,
                            FindAllWebAccountsStatus_NotAllowedByProvider = 1,
                            FindAllWebAccountsStatus_NotSupportedByProvider = 2,
                            FindAllWebAccountsStatus_ProviderError = 3,
                        };
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        enum WebAuthenticationAddAccountStatus : int
                        {
                            WebAuthenticationAddAccountStatus_Success = 0,
                            WebAuthenticationAddAccountStatus_Error = 1,
                            WebAuthenticationAddAccountStatus_NotSupportedByProvider = 2,
                            WebAuthenticationAddAccountStatus_ServiceConnectionError = 3,
                            WebAuthenticationAddAccountStatus_ProviderError = 4,
                        };
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType
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
                    namespace Core {
                        enum WebTokenRequestPromptType : int
                        {
                            WebTokenRequestPromptType_Default = 0,
                            WebTokenRequestPromptType_ForceAuthentication = 1,
                        };
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebTokenRequestStatus
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
                    namespace Core {
                        enum WebTokenRequestStatus : int
                        {
                            WebTokenRequestStatus_Success = 0,
                            WebTokenRequestStatus_UserCancel = 1,
                            WebTokenRequestStatus_AccountSwitch = 2,
                            WebTokenRequestStatus_UserInteractionRequired = 3,
                            WebTokenRequestStatus_AccountProviderNotAvailable = 4,
                            WebTokenRequestStatus_ProviderError = 5,
                        };
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IFindAllAccountsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.FindAllAccountsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IFindAllAccountsResult[] = L"Windows.Security.Authentication.Web.Core.IFindAllAccountsResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("a5812b5d-b72e-420c-86ab-aac0d7b7261f")
                        IFindAllAccountsResult : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Accounts(
                                __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::Security::Authentication::Web::Core::FindAllWebAccountsStatus* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ProviderError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IFindAllAccountsResult = __uuidof(IFindAllAccountsResult);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountEventArgs[] = L"Windows.Security.Authentication.Web.Core.IWebAccountEventArgs";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("6fb7037d-424e-44ec-977c-ef2415462a5a")
                        IWebAccountEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Account(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountEventArgs = __uuidof(IWebAccountEventArgs);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountMonitor[] = L"Windows.Security.Authentication.Web.Core.IWebAccountMonitor";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("7445f5fd-aa9d-4619-8d5d-c138a4ede3e5")
                        IWebAccountMonitor : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE add_Updated(
                                __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Updated(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_Removed(
                                __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Removed(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_DefaultSignInAccountChanged(
                                __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_DefaultSignInAccountChanged(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountMonitor = __uuidof(IWebAccountMonitor);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountMonitor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountMonitor2[] = L"Windows.Security.Authentication.Web.Core.IWebAccountMonitor2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("a7adc1f8-24b8-4f01-9ae5-24545e71233a")
                        IWebAccountMonitor2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE add_AccountPictureUpdated(
                                __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_AccountPictureUpdated(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountMonitor2 = __uuidof(IWebAccountMonitor2);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResponse[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("7fb013e8-0bd8-542b-b486-8323163a4b85")
                        IWebAuthenticationAddAccountResponse : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Properties(
                                __FIMap_2_HSTRING_HSTRING** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationAddAccountResponse = __uuidof(IWebAuthenticationAddAccountResponse);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResponseFactory[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("325f903e-77be-5365-81d9-0321cdd82195")
                        IWebAuthenticationAddAccountResponseFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateWithAccount(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResponse** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationAddAccountResponseFactory = __uuidof(IWebAuthenticationAddAccountResponseFactory);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResult[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("88fad03c-901d-5ffa-9259-701d3ca08ef2")
                        IWebAuthenticationAddAccountResult : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseData(
                                ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationAddAccountResponse** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseStatus(
                                ABI::Windows::Security::Authentication::Web::Core::WebAuthenticationAddAccountStatus* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationAddAccountResult = __uuidof(IWebAuthenticationAddAccountResult);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("6aca7c92-a581-4479-9c10-752eff44fd34")
                        IWebAuthenticationCoreManagerStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetTokenSilentlyAsync(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetTokenSilentlyWithWebAccountAsync(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RequestTokenAsync(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RequestTokenWithWebAccountAsync(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest* request,
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING webAccountId,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindAccountProviderAsync(
                                HSTRING webAccountProviderId,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindAccountProviderWithAuthorityAsync(
                                HSTRING webAccountProviderId,
                                HSTRING authority,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerStatics = __uuidof(IWebAuthenticationCoreManagerStatics);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics2[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("f584184a-8b57-4820-b6a4-70a5b6fcf44a")
                        IWebAuthenticationCoreManagerStatics2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE FindAccountProviderWithAuthorityForUserAsync(
                                HSTRING webAccountProviderId,
                                HSTRING authority,
                                ABI::Windows::System::IUser* user,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerStatics2 = __uuidof(IWebAuthenticationCoreManagerStatics2);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics3[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("2404eeb2-8924-4d93-ab3a-99688b419d56")
                        IWebAuthenticationCoreManagerStatics3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateWebAccountMonitor(
                                __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* webAccounts,
                                ABI::Windows::Security::Authentication::Web::Core::IWebAccountMonitor** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerStatics3 = __uuidof(IWebAuthenticationCoreManagerStatics3);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics4[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("54e633fe-96e0-41e8-9832-1298897c2aaf")
                        IWebAuthenticationCoreManagerStatics4 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE FindAllAccountsAsync(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindAllAccountsWithClientIdAsync(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING clientId,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindSystemAccountProviderAsync(
                                HSTRING webAccountProviderId,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindSystemAccountProviderWithAuthorityAsync(
                                HSTRING webAccountProviderId,
                                HSTRING authority,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindSystemAccountProviderWithAuthorityForUserAsync(
                                HSTRING webAccountProviderId,
                                HSTRING authority,
                                ABI::Windows::System::IUser* user,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerStatics4 = __uuidof(IWebAuthenticationCoreManagerStatics4);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics5[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("d07c1ded-270f-4554-9966-27b7df05b965")
                        IWebAuthenticationCoreManagerStatics5 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE AddAccountWithTransferTokenAsync(
                                ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationTransferTokenRequest* request,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationCoreManagerStatics5 = __uuidof(IWebAuthenticationCoreManagerStatics5);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationTransferTokenRequest[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("7acfa5b6-529d-5e76-9846-f3fd999304d0")
                        IWebAuthenticationTransferTokenRequest : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccountProvider(
                                ABI::Windows::Security::Credentials::IWebAccountProvider** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TransferToken(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TransferToken(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Properties(
                                __FIMap_2_HSTRING_HSTRING** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CorrelationId(
                                HSTRING value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationTransferTokenRequest = __uuidof(IWebAuthenticationTransferTokenRequest);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationTransferTokenRequestFactory[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("5f16b627-04c4-5f0b-8683-8bab58965656")
                        IWebAuthenticationTransferTokenRequestFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING transferToken,
                                ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationTransferTokenRequest** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithCorrelationId(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING transferToken,
                                HSTRING correlationId,
                                ABI::Windows::Security::Authentication::Web::Core::IWebAuthenticationTransferTokenRequest** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAuthenticationTransferTokenRequestFactory = __uuidof(IWebAuthenticationTransferTokenRequestFactory);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebProviderError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebProviderError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebProviderError[] = L"Windows.Security.Authentication.Web.Core.IWebProviderError";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("db191bb1-50c5-4809-8dca-09c99410245c")
                        IWebProviderError : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                                UINT32* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ErrorMessage(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Properties(
                                __FIMap_2_HSTRING_HSTRING** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderError = __uuidof(IWebProviderError);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebProviderError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebProviderErrorFactory[] = L"Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("e3c40a2d-89ef-4e37-847f-a8b9d5a32910")
                        IWebProviderErrorFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                UINT32 errorCode,
                                HSTRING errorMessage,
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError** webProviderError
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderErrorFactory = __uuidof(IWebProviderErrorFactory);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("b77b4d68-adcb-4673-b364-0cf7b35caf97")
                        IWebTokenRequest : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccountProvider(
                                ABI::Windows::Security::Credentials::IWebAccountProvider** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Scope(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ClientId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PromptType(
                                ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestPromptType* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Properties(
                                __FIMap_2_HSTRING_HSTRING** requestProperties
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenRequest = __uuidof(IWebTokenRequest);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest2[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("d700c079-30c8-4397-9654-961c3be8b855")
                        IWebTokenRequest2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AppProperties(
                                __FIMap_2_HSTRING_HSTRING** requestProperties
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenRequest2 = __uuidof(IWebTokenRequest2);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest3[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("5a755b51-3bb1-41a5-a63d-90bc32c7db9a")
                        IWebTokenRequest3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CorrelationId(
                                HSTRING value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenRequest3 = __uuidof(IWebTokenRequest3);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequestFactory[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("6cf2141c-0ff0-4c67-b84f-99ddbe4a72c9")
                        IWebTokenRequestFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING scope,
                                HSTRING clientId,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest** webTokenRequest
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithPromptType(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING scope,
                                HSTRING clientId,
                                ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestPromptType promptType,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest** webTokenRequest
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithProvider(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest** webTokenRequest
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithScope(
                                ABI::Windows::Security::Credentials::IWebAccountProvider* provider,
                                HSTRING scope,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest** webTokenRequest
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenRequestFactory = __uuidof(IWebTokenRequestFactory);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequestResult[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequestResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("c12a8305-d1f8-4483-8d54-38fe292784ff")
                        IWebTokenRequestResult : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseData(
                                __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseStatus(
                                ABI::Windows::Security::Authentication::Web::Core::WebTokenRequestStatus* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ResponseError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InvalidateCacheAsync(
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenRequestResult = __uuidof(IWebTokenRequestResult);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenResponse[] = L"Windows.Security.Authentication.Web.Core.IWebTokenResponse";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("67a7c5ca-83f6-44c6-a3b1-0eb69e41fa8a")
                        IWebTokenResponse : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Token(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ProviderError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Properties(
                                __FIMap_2_HSTRING_HSTRING** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenResponse = __uuidof(IWebTokenResponse);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenResponseFactory[] = L"Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Core {
                        MIDL_INTERFACE("ab6bf7f8-5450-4ef6-97f7-052b0431c0f0")
                        IWebTokenResponseFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateWithToken(
                                HSTRING token,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse** webTokenResponse
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithTokenAndAccount(
                                HSTRING token,
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse** webTokenResponse
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithTokenAccountAndError(
                                HSTRING token,
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError* error,
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse** webTokenResponse
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebTokenResponseFactory = __uuidof(IWebTokenResponseFactory);
                    } /* Core */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.FindAllAccountsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IFindAllAccountsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_FindAllAccountsResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_FindAllAccountsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_FindAllAccountsResult[] = L"Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAccountEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAccountEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAccountEventArgs[] = L"Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAccountMonitor ** Default Interface **
 *    Windows.Security.Authentication.Web.Core.IWebAccountMonitor2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountMonitor_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAccountMonitor[] = L"Windows.Security.Authentication.Web.Core.WebAccountMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebProviderError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebProviderError ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebProviderError_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebProviderError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebProviderError[] = L"Windows.Security.Authentication.Web.Core.WebProviderError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest ** Default Interface **
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest2
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenRequest[] = L"Windows.Security.Authentication.Web.Core.WebTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenRequestResult[] = L"Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenResponse[] = L"Windows.Security.Authentication.Web.Core.WebTokenResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CWebAccountProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIIterator_1_Windows__CSecurity__CCredentials__CWebAccount** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* sender,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CFindAllWebAccountsStatus __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CFindAllWebAccountsStatus;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebAuthenticationAddAccountStatus __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebAuthenticationAddAccountStatus;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestPromptType __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestPromptType;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestStatus __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestStatus;

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CFindAllWebAccountsStatus
{
    FindAllWebAccountsStatus_Success = 0,
    FindAllWebAccountsStatus_NotAllowedByProvider = 1,
    FindAllWebAccountsStatus_NotSupportedByProvider = 2,
    FindAllWebAccountsStatus_ProviderError = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebAuthenticationAddAccountStatus
{
    WebAuthenticationAddAccountStatus_Success = 0,
    WebAuthenticationAddAccountStatus_Error = 1,
    WebAuthenticationAddAccountStatus_NotSupportedByProvider = 2,
    WebAuthenticationAddAccountStatus_ServiceConnectionError = 3,
    WebAuthenticationAddAccountStatus_ProviderError = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestPromptType
{
    WebTokenRequestPromptType_Default = 0,
    WebTokenRequestPromptType_ForceAuthentication = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Core.WebTokenRequestStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestStatus
{
    WebTokenRequestStatus_Success = 0,
    WebTokenRequestStatus_UserCancel = 1,
    WebTokenRequestStatus_AccountSwitch = 2,
    WebTokenRequestStatus_UserInteractionRequired = 3,
    WebTokenRequestStatus_AccountProviderNotAvailable = 4,
    WebTokenRequestStatus_ProviderError = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IFindAllAccountsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.FindAllAccountsResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IFindAllAccountsResult[] = L"Windows.Security.Authentication.Web.Core.IFindAllAccountsResult";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Accounts)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CFindAllWebAccountsStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_get_Accounts(This, value) \
    ((This)->lpVtbl->get_Accounts(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_get_ProviderError(This, value) \
    ((This)->lpVtbl->get_ProviderError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIFindAllAccountsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountEventArgs[] = L"Windows.Security.Authentication.Web.Core.IWebAccountEventArgs";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Account)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_get_Account(This, value) \
    ((This)->lpVtbl->get_Account(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountMonitor[] = L"Windows.Security.Authentication.Web.Core.IWebAccountMonitor";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Updated)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Updated)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Removed)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Removed)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DefaultSignInAccountChanged)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DefaultSignInAccountChanged)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitorVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_add_Updated(This, handler, token) \
    ((This)->lpVtbl->add_Updated(This, handler, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_remove_Updated(This, token) \
    ((This)->lpVtbl->remove_Updated(This, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_add_Removed(This, handler, token) \
    ((This)->lpVtbl->add_Removed(This, handler, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_remove_Removed(This, token) \
    ((This)->lpVtbl->remove_Removed(This, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_add_DefaultSignInAccountChanged(This, handler, token) \
    ((This)->lpVtbl->add_DefaultSignInAccountChanged(This, handler, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_remove_DefaultSignInAccountChanged(This, token) \
    ((This)->lpVtbl->remove_DefaultSignInAccountChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAccountMonitor2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAccountMonitor2[] = L"Windows.Security.Authentication.Web.Core.IWebAccountMonitor2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AccountPictureUpdated)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        __FITypedEventHandler_2_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountMonitor_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAccountEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AccountPictureUpdated)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_add_AccountPictureUpdated(This, handler, token) \
    ((This)->lpVtbl->add_AccountPictureUpdated(This, handler, token))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_remove_AccountPictureUpdated(This, token) \
    ((This)->lpVtbl->remove_AccountPictureUpdated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResponse[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse* This,
        __FIMap_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResponseFactory[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_CreateWithAccount(This, webAccount, value) \
    ((This)->lpVtbl->CreateWithAccount(This, webAccount, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationAddAccountResult[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResponseData)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResponse** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseStatus)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebAuthenticationAddAccountStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_get_ResponseData(This, value) \
    ((This)->lpVtbl->get_ResponseData(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_get_ResponseStatus(This, value) \
    ((This)->lpVtbl->get_ResponseStatus(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_get_ResponseError(This, value) \
    ((This)->lpVtbl->get_ResponseError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationAddAccountResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTokenSilentlyAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetTokenSilentlyWithWebAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* RequestTokenAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* RequestTokenWithWebAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* request,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenRequestResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FindAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING webAccountId,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FindAccountProviderAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        HSTRING webAccountProviderId,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FindAccountProviderWithAuthorityAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics* This,
        HSTRING webAccountProviderId,
        HSTRING authority,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_GetTokenSilentlyAsync(This, request, asyncInfo) \
    ((This)->lpVtbl->GetTokenSilentlyAsync(This, request, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_GetTokenSilentlyWithWebAccountAsync(This, request, webAccount, asyncInfo) \
    ((This)->lpVtbl->GetTokenSilentlyWithWebAccountAsync(This, request, webAccount, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_RequestTokenAsync(This, request, asyncInfo) \
    ((This)->lpVtbl->RequestTokenAsync(This, request, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_RequestTokenWithWebAccountAsync(This, request, webAccount, asyncInfo) \
    ((This)->lpVtbl->RequestTokenWithWebAccountAsync(This, request, webAccount, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FindAccountAsync(This, provider, webAccountId, asyncInfo) \
    ((This)->lpVtbl->FindAccountAsync(This, provider, webAccountId, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FindAccountProviderAsync(This, webAccountProviderId, asyncInfo) \
    ((This)->lpVtbl->FindAccountProviderAsync(This, webAccountProviderId, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_FindAccountProviderWithAuthorityAsync(This, webAccountProviderId, authority, asyncInfo) \
    ((This)->lpVtbl->FindAccountProviderWithAuthorityAsync(This, webAccountProviderId, authority, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics2[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAccountProviderWithAuthorityForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2* This,
        HSTRING webAccountProviderId,
        HSTRING authority,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_FindAccountProviderWithAuthorityForUserAsync(This, webAccountProviderId, authority, user, asyncInfo) \
    ((This)->lpVtbl->FindAccountProviderWithAuthorityForUserAsync(This, webAccountProviderId, authority, user, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics3[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWebAccountMonitor)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3* This,
        __FIIterable_1_Windows__CSecurity__CCredentials__CWebAccount* webAccounts,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAccountMonitor** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_CreateWebAccountMonitor(This, webAccounts, result) \
    ((This)->lpVtbl->CreateWebAccountMonitor(This, webAccounts, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics4[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAccountsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllAccountsWithClientIdAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING clientId,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CFindAllAccountsResult** operation);
    HRESULT (STDMETHODCALLTYPE* FindSystemAccountProviderAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        HSTRING webAccountProviderId,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation);
    HRESULT (STDMETHODCALLTYPE* FindSystemAccountProviderWithAuthorityAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        HSTRING webAccountProviderId,
        HSTRING authority,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation);
    HRESULT (STDMETHODCALLTYPE* FindSystemAccountProviderWithAuthorityForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4* This,
        HSTRING webAccountProviderId,
        HSTRING authority,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccountProvider** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FindAllAccountsAsync(This, provider, operation) \
    ((This)->lpVtbl->FindAllAccountsAsync(This, provider, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FindAllAccountsWithClientIdAsync(This, provider, clientId, operation) \
    ((This)->lpVtbl->FindAllAccountsWithClientIdAsync(This, provider, clientId, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FindSystemAccountProviderAsync(This, webAccountProviderId, operation) \
    ((This)->lpVtbl->FindSystemAccountProviderAsync(This, webAccountProviderId, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FindSystemAccountProviderWithAuthorityAsync(This, webAccountProviderId, authority, operation) \
    ((This)->lpVtbl->FindSystemAccountProviderWithAuthorityAsync(This, webAccountProviderId, authority, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_FindSystemAccountProviderWithAuthorityForUserAsync(This, webAccountProviderId, authority, user, operation) \
    ((This)->lpVtbl->FindSystemAccountProviderWithAuthorityForUserAsync(This, webAccountProviderId, authority, user, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationCoreManagerStatics5[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddAccountWithTransferTokenAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebAuthenticationAddAccountResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_AddAccountWithTransferTokenAsync(This, request, operation) \
    ((This)->lpVtbl->AddAccountWithTransferTokenAsync(This, request, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationCoreManagerStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationTransferTokenRequest[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountProvider)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_TransferToken)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_TransferToken)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CorrelationId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_get_WebAccountProvider(This, value) \
    ((This)->lpVtbl->get_WebAccountProvider(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_get_TransferToken(This, value) \
    ((This)->lpVtbl->get_TransferToken(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_put_TransferToken(This, value) \
    ((This)->lpVtbl->put_TransferToken(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_get_CorrelationId(This, value) \
    ((This)->lpVtbl->get_CorrelationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_put_CorrelationId(This, value) \
    ((This)->lpVtbl->put_CorrelationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebAuthenticationTransferTokenRequestFactory[] = L"Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING transferToken,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithCorrelationId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING transferToken,
        HSTRING correlationId,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_Create(This, provider, transferToken, value) \
    ((This)->lpVtbl->Create(This, provider, transferToken, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_CreateWithCorrelationId(This, provider, transferToken, correlationId, value) \
    ((This)->lpVtbl->CreateWithCorrelationId(This, provider, transferToken, correlationId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebAuthenticationTransferTokenRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebProviderError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebProviderError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebProviderError[] = L"Windows.Security.Authentication.Web.Core.IWebProviderError";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorMessage)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* This,
        __FIMap_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_get_ErrorMessage(This, value) \
    ((This)->lpVtbl->get_ErrorMessage(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebProviderError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebProviderErrorFactory[] = L"Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory* This,
        UINT32 errorCode,
        HSTRING errorMessage,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError** webProviderError);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_Create(This, errorCode, errorMessage, webProviderError) \
    ((This)->lpVtbl->Create(This, errorCode, errorMessage, webProviderError))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderErrorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountProvider)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_Scope)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ClientId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PromptType)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestPromptType* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest* This,
        __FIMap_2_HSTRING_HSTRING** requestProperties);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_get_WebAccountProvider(This, value) \
    ((This)->lpVtbl->get_WebAccountProvider(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_get_Scope(This, value) \
    ((This)->lpVtbl->get_Scope(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_get_ClientId(This, value) \
    ((This)->lpVtbl->get_ClientId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_get_PromptType(This, value) \
    ((This)->lpVtbl->get_PromptType(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_get_Properties(This, requestProperties) \
    ((This)->lpVtbl->get_Properties(This, requestProperties))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest2[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppProperties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2* This,
        __FIMap_2_HSTRING_HSTRING** requestProperties);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_get_AppProperties(This, requestProperties) \
    ((This)->lpVtbl->get_AppProperties(This, requestProperties))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequest3[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequest3";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CorrelationId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_get_CorrelationId(This, value) \
    ((This)->lpVtbl->get_CorrelationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_put_CorrelationId(This, value) \
    ((This)->lpVtbl->put_CorrelationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequestFactory[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING scope,
        HSTRING clientId,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest** webTokenRequest);
    HRESULT (STDMETHODCALLTYPE* CreateWithPromptType)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING scope,
        HSTRING clientId,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestPromptType promptType,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest** webTokenRequest);
    HRESULT (STDMETHODCALLTYPE* CreateWithProvider)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest** webTokenRequest);
    HRESULT (STDMETHODCALLTYPE* CreateWithScope)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* provider,
        HSTRING scope,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest** webTokenRequest);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_Create(This, provider, scope, clientId, webTokenRequest) \
    ((This)->lpVtbl->Create(This, provider, scope, clientId, webTokenRequest))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_CreateWithPromptType(This, provider, scope, clientId, promptType, webTokenRequest) \
    ((This)->lpVtbl->CreateWithPromptType(This, provider, scope, clientId, promptType, webTokenRequest))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_CreateWithProvider(This, provider, webTokenRequest) \
    ((This)->lpVtbl->CreateWithProvider(This, provider, webTokenRequest))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_CreateWithScope(This, provider, scope, webTokenRequest) \
    ((This)->lpVtbl->CreateWithScope(This, provider, scope, webTokenRequest))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenRequestResult[] = L"Windows.Security.Authentication.Web.Core.IWebTokenRequestResult";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResponseData)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CCore__CWebTokenResponse** value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseStatus)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CWebTokenRequestStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError** value);
    HRESULT (STDMETHODCALLTYPE* InvalidateCacheAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_get_ResponseData(This, value) \
    ((This)->lpVtbl->get_ResponseData(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_get_ResponseStatus(This, value) \
    ((This)->lpVtbl->get_ResponseStatus(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_get_ResponseError(This, value) \
    ((This)->lpVtbl->get_ResponseError(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_InvalidateCacheAsync(This, asyncInfo) \
    ((This)->lpVtbl->InvalidateCacheAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenResponse[] = L"Windows.Security.Authentication.Web.Core.IWebTokenResponse";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Token)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError** value);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* This,
        __FIMap_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_get_Token(This, value) \
    ((This)->lpVtbl->get_Token(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_get_ProviderError(This, value) \
    ((This)->lpVtbl->get_ProviderError(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Core_IWebTokenResponseFactory[] = L"Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithToken)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        HSTRING token,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** webTokenResponse);
    HRESULT (STDMETHODCALLTYPE* CreateWithTokenAndAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        HSTRING token,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** webTokenResponse);
    HRESULT (STDMETHODCALLTYPE* CreateWithTokenAccountAndError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory* This,
        HSTRING token,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* error,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** webTokenResponse);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_CreateWithToken(This, token, webTokenResponse) \
    ((This)->lpVtbl->CreateWithToken(This, token, webTokenResponse))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_CreateWithTokenAndAccount(This, token, webAccount, webTokenResponse) \
    ((This)->lpVtbl->CreateWithTokenAndAccount(This, token, webAccount, webTokenResponse))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_CreateWithTokenAccountAndError(This, token, webAccount, error, webTokenResponse) \
    ((This)->lpVtbl->CreateWithTokenAccountAndError(This, token, webAccount, error, webTokenResponse))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.FindAllAccountsResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IFindAllAccountsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_FindAllAccountsResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_FindAllAccountsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_FindAllAccountsResult[] = L"Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAccountEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAccountEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAccountEventArgs[] = L"Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAccountMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAccountMonitor ** Default Interface **
 *    Windows.Security.Authentication.Web.Core.IWebAccountMonitor2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountMonitor_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAccountMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAccountMonitor[] = L"Windows.Security.Authentication.Web.Core.WebAccountMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponseFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResponse[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationAddAccountResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationAddAccountResult[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationAddAccountResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics4 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Core.IWebAuthenticationCoreManagerStatics5 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationCoreManager[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequestFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebAuthenticationTransferTokenRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebAuthenticationTransferTokenRequest[] = L"Windows.Security.Authentication.Web.Core.WebAuthenticationTransferTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebProviderError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebProviderErrorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebProviderError ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebProviderError_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebProviderError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebProviderError[] = L"Windows.Security.Authentication.Web.Core.WebProviderError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebTokenRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest ** Default Interface **
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest2
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequest3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenRequest[] = L"Windows.Security.Authentication.Web.Core.WebTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenRequestResult[] = L"Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Core.WebTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Security.Authentication.Web.Core.IWebTokenResponseFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Core.IWebTokenResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Core_WebTokenResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Core_WebTokenResponse[] = L"Windows.Security.Authentication.Web.Core.WebTokenResponse";
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
#endif // __windows2Esecurity2Eauthentication2Eweb2Ecore_p_h__

#endif // __windows2Esecurity2Eauthentication2Eweb2Ecore_h__
