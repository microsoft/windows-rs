
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
#ifndef __windows2Esecurity2Eauthentication2Eweb2Eprovider_h__
#define __windows2Esecurity2Eauthentication2Eweb2Eprovider_h__
#ifndef __windows2Esecurity2Eauthentication2Eweb2Eprovider_p_h__
#define __windows2Esecurity2Eauthentication2Eweb2Eprovider_p_h__


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
#include "Windows.Security.Authentication.Web.h"
#include "Windows.Security.Authentication.Web.Core.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Security.Cryptography.Core.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountClientView;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountClientViewFactory;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientViewFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountManagerStatics;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics ABI::Windows::Security::Authentication::Web::Provider::IWebAccountManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountManagerStatics2;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2 ABI::Windows::Security::Authentication::Web::Provider::IWebAccountManagerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountManagerStatics3;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3 ABI::Windows::Security::Authentication::Web::Provider::IWebAccountManagerStatics3

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountManagerStatics4;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4 ABI::Windows::Security::Authentication::Web::Provider::IWebAccountManagerStatics4

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountMapManagerStatics;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics ABI::Windows::Security::Authentication::Web::Provider::IWebAccountMapManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderAddAccountOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderAddAccountOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderBaseReportOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderBaseReportOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderDeleteAccountOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderDeleteAccountOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderManageAccountOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderManageAccountOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderRetrieveCookiesOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderRetrieveCookiesOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderSignOutAccountOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderSignOutAccountOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderSilentReportOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderSilentReportOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderTokenObjects;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderTokenObjects

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderTokenObjects2;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2 ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderTokenObjects2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderTokenOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderTokenOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountProviderUIReportOperation;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderUIReportOperation

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebAccountScopeManagerStatics;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics ABI::Windows::Security::Authentication::Web::Provider::IWebAccountScopeManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebProviderTokenRequest;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenRequest

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebProviderTokenRequest2;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2 ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenRequest2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebProviderTokenRequest3;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3 ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenRequest3

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebProviderTokenResponse;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        interface IWebProviderTokenResponseFactory;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponseFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__

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
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        class WebAccountClientView;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a5984607-661d-5e9c-a0ba-5c7d5f41af1c"))
IIterator<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*, ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.Web.Provider.WebAccountClientView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("610e0f9d-aae4-54e1-bb0b-1aca14110abf"))
IIterable<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*, ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.Web.Provider.WebAccountClientView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3dfd5eff-8fa4-5e9d-8d1c-0f18d542be35"))
IVectorView<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*, ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Web.Provider.WebAccountClientView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientView*> __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("116827c1-187e-5095-a14b-df4111c638c2"))
IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Web.Provider.WebAccountClientView>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*> __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3fa6536f-7e7a-5bc9-b20f-d866cacaf81c"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Web.Provider.WebAccountClientView>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_USE */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("66b59040-7c93-5f96-b52f-2c098d1557d0"))
IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Credentials.WebAccount>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*> __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c2090d8c-37d8-5c47-9581-0f17b91a0cd3"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Credentials.WebAccount>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            namespace Cryptography {
                namespace Core {
                    class CryptographicKey;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Core {
                    interface ICryptographicKey;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey ABI::Windows::Security::Cryptography::Core::ICryptographicKey

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("81ca789b-98df-5c6a-9531-966238e3e7ae"))
IAsyncOperation<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Core::CryptographicKey*, ABI::Windows::Security::Cryptography::Core::ICryptographicKey*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Cryptography.Core.CryptographicKey>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("04ca4378-f594-5de6-a555-304f62cb4faf"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Core::CryptographicKey*, ABI::Windows::Security::Cryptography::Core::ICryptographicKey*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Cryptography.Core.CryptographicKey>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Cryptography::Core::CryptographicKey*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bee8834-b9a7-5a80-a746-5ef097227878"))
IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c3d2fd-b8a1-5620-b746-7ee6d533aca3"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IBuffer*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_USE */

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
                    namespace Provider {
                        class WebProviderTokenResponse;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb57825d-5ad6-5ee0-8dc6-a53c1e82e3ab"))
IIterator<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*, ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bac236-c077-553a-b4ae-b58fb0b89918"))
IIterable<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*, ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE */

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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1ae644b7-9839-585b-8792-ecd5050b88bb"))
IVectorView<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*, ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#define DEF___FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e7ad5cf-390f-5ecd-b714-3c654b84cbba"))
IVector<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*, ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Security::Authentication::Web::Provider::WebProviderTokenResponse*> __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t;
#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CHttp__CHttpCookie_USE
#define DEF___FIVector_1_Windows__CWeb__CHttp__CHttpCookie_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98a6c2fe-469b-5bdd-a16d-7002c3a0853d"))
IVector<ABI::Windows::Web::Http::HttpCookie*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::Http::HttpCookie*, ABI::Windows::Web::Http::IHttpCookie*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.Http.HttpCookie>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Web::Http::HttpCookie*> __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_t;
#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CHttp__CHttpCookie_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CHttp__CHttpCookie_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    typedef enum TokenBindingKeyType : int TokenBindingKeyType;
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

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
                    namespace Provider {
                        typedef enum WebAccountClientViewType : int WebAccountClientViewType;
                    } /* Provider */
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
                    namespace Provider {
                        typedef enum WebAccountProviderOperationKind : int WebAccountProviderOperationKind;
                    } /* Provider */
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
                    namespace Provider {
                        typedef enum WebAccountScope : int WebAccountScope;
                    } /* Provider */
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
                    namespace Provider {
                        typedef enum WebAccountSelectionOptions : unsigned int WebAccountSelectionOptions;
                    } /* Provider */
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
                    namespace Provider {
                        class WebProviderTokenRequest;
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountClientViewType
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
                    namespace Provider {
                        enum WebAccountClientViewType : int
                        {
                            WebAccountClientViewType_IdOnly = 0,
                            WebAccountClientViewType_IdAndProperties = 1,
                        };
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountProviderOperationKind
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
                    namespace Provider {
                        enum WebAccountProviderOperationKind : int
                        {
                            WebAccountProviderOperationKind_RequestToken = 0,
                            WebAccountProviderOperationKind_GetTokenSilently = 1,
                            WebAccountProviderOperationKind_AddAccount = 2,
                            WebAccountProviderOperationKind_ManageAccount = 3,
                            WebAccountProviderOperationKind_DeleteAccount = 4,
                            WebAccountProviderOperationKind_RetrieveCookies = 5,
                            WebAccountProviderOperationKind_SignOutAccount = 6,
                        };
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountScope
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
                    namespace Provider {
                        enum WebAccountScope : int
                        {
                            WebAccountScope_PerUser = 0,
                            WebAccountScope_PerApplication = 1,
                        };
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountSelectionOptions
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
                    namespace Provider {
                        enum WebAccountSelectionOptions : unsigned int
                        {
                            WebAccountSelectionOptions_Default = 0,
                            WebAccountSelectionOptions_New = 0x1,
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(WebAccountSelectionOptions)
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountClientView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountClientView[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountClientView";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("e7bd66ba-0bc7-4c66-bfd4-65d3082cbca8")
                        IWebAccountClientView : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationCallbackUri(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Type(
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientViewType* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AccountPairwiseId(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountClientView = __uuidof(IWebAccountClientView);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountClientViewFactory[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("616d16a4-de22-4855-a326-06cebf2a3f23")
                        IWebAccountClientViewFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientViewType viewType,
                                ABI::Windows::Foundation::IUriRuntimeClass* applicationCallbackUri,
                                ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView** view
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateWithPairwiseId(
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountClientViewType viewType,
                                ABI::Windows::Foundation::IUriRuntimeClass* applicationCallbackUri,
                                HSTRING accountPairwiseId,
                                ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView** view
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountClientViewFactory = __uuidof(IWebAccountClientViewFactory);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("b2e8e1a6-d49a-4032-84bf-1a2847747bf1")
                        IWebAccountManagerStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE UpdateWebAccountPropertiesAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* additionalProperties,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountAsync(
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE DeleteWebAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE FindAllProviderWebAccountsAsync(
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE PushCookiesAsync(
                                ABI::Windows::Foundation::IUriRuntimeClass* uri,
                                __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* cookies,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetViewAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Provider::IWebAccountClientView* view,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearViewAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Foundation::IUriRuntimeClass* applicationCallbackUri,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetViewsAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetWebAccountPictureAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Storage::Streams::IRandomAccessStream* webAccountPicture,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearWebAccountPictureAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountManagerStatics = __uuidof(IWebAccountManagerStatics);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics2[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("68a7a829-2d5f-4653-8bb0-bd2fa6bd2d87")
                        IWebAccountManagerStatics2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE PullCookiesAsync(
                                HSTRING uriString,
                                HSTRING callerPFN,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountManagerStatics2 = __uuidof(IWebAccountManagerStatics2);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics3[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("dd4523a6-8a4f-4aa2-b15e-03f550af1359")
                        IWebAccountManagerStatics3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE FindAllProviderWebAccountsForUserAsync(
                                ABI::Windows::System::IUser* user,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountWithScopeForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope scope,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountWithScopeAndMapForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope scope,
                                HSTRING perUserWebAccountId,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountManagerStatics3 = __uuidof(IWebAccountManagerStatics3);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics4[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("59ebc2d2-f7db-412f-bc3f-f2fea04430b4")
                        IWebAccountManagerStatics4 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE InvalidateAppCacheForAllAccountsAsync(
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE InvalidateAppCacheForAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountManagerStatics4 = __uuidof(IWebAccountManagerStatics4);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountMapManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("e8fa446f-3a1b-48a4-8e90-1e59ca6f54db")
                        IWebAccountMapManagerStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountWithScopeAndMapAsync(
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope scope,
                                HSTRING perUserWebAccountId,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetPerAppToPerUserAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* perAppAccount,
                                HSTRING perUserWebAccountId,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetPerUserFromPerAppAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* perAppAccount,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ClearPerUserFromPerAppAccountAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* perAppAccount,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountMapManagerStatics = __uuidof(IWebAccountMapManagerStatics);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderAddAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("73ebdccf-4378-4c79-9335-a5d7ab81594e")
                        IWebAccountProviderAddAccountOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderAddAccountOperation = __uuidof(IWebAccountProviderAddAccountOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderBaseReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("bba4acbb-993b-4d57-bbe4-1421e3668b4c")
                        IWebAccountProviderBaseReportOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ReportError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderBaseReportOperation = __uuidof(IWebAccountProviderBaseReportOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderDeleteAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("0abb48b8-9e01-49c9-a355-7d48caf7d6ca")
                        IWebAccountProviderDeleteAccountOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderDeleteAccountOperation = __uuidof(IWebAccountProviderDeleteAccountOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderManageAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("ed20dc5c-d21b-463e-a9b7-c1fd0edae978")
                        IWebAccountProviderManageAccountOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderManageAccountOperation = __uuidof(IWebAccountProviderManageAccountOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("6d5d2426-10b1-419a-a44e-f9c5161574e6")
                        IWebAccountProviderOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Kind(
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountProviderOperationKind* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderOperation = __uuidof(IWebAccountProviderOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderRetrieveCookiesOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("5a040441-0fa3-4ab1-a01c-20b110358594")
                        IWebAccountProviderRetrieveCookiesOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Context(
                                ABI::Windows::Foundation::IUriRuntimeClass** webCookieRequestContext
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Cookies(
                                __FIVector_1_Windows__CWeb__CHttp__CHttpCookie** cookies
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Uri(
                                ABI::Windows::Foundation::IUriRuntimeClass* uri
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Uri(
                                ABI::Windows::Foundation::IUriRuntimeClass** uri
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationCallbackUri(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderRetrieveCookiesOperation = __uuidof(IWebAccountProviderRetrieveCookiesOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderSignOutAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("b890e21d-0c55-47bc-8c72-04a6fc7cac07")
                        IWebAccountProviderSignOutAccountOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                                ABI::Windows::Security::Credentials::IWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationCallbackUri(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ClientId(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderSignOutAccountOperation = __uuidof(IWebAccountProviderSignOutAccountOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderSilentReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("e0b545f8-3b0f-44da-924c-7b18baaa62a9")
                        IWebAccountProviderSilentReportOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE ReportUserInteractionRequired(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE ReportUserInteractionRequiredWithError(
                                ABI::Windows::Security::Authentication::Web::Core::IWebProviderError* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderSilentReportOperation = __uuidof(IWebAccountProviderSilentReportOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenObjects[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("408f284b-1328-42db-89a4-0bce7a717d8e")
                        IWebAccountProviderTokenObjects : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Operation(
                                ABI::Windows::Security::Authentication::Web::Provider::IWebAccountProviderOperation** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderTokenObjects = __uuidof(IWebAccountProviderTokenObjects);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenObjects2[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("1020b893-5ca5-4fff-95fb-b820273fc395")
                        IWebAccountProviderTokenObjects2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_User(
                                ABI::Windows::System::IUser** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderTokenObjects2 = __uuidof(IWebAccountProviderTokenObjects2);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("95c613be-2034-4c38-9434-d26c14b2b4b2")
                        IWebAccountProviderTokenOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ProviderRequest(
                                ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenRequest** webTokenRequest
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ProviderResponses(
                                __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CacheExpirationTime(
                                ABI::Windows::Foundation::DateTime value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CacheExpirationTime(
                                ABI::Windows::Foundation::DateTime* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderTokenOperation = __uuidof(IWebAccountProviderTokenOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderUIReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("28ff92d3-8f80-42fb-944f-b2107bbd42e6")
                        IWebAccountProviderUIReportOperation : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE ReportUserCanceled(void) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountProviderUIReportOperation = __uuidof(IWebAccountProviderUIReportOperation);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountScopeManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("5c6ce37c-12b2-423a-bf3d-85b8d7e53656")
                        IWebAccountScopeManagerStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE AddWebAccountWithScopeAsync(
                                HSTRING webAccountId,
                                HSTRING webAccountUserName,
                                __FIMapView_2_HSTRING_HSTRING* props,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope scope,
                                __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SetScopeAsync(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope scope,
                                ABI::Windows::Foundation::IAsyncAction** asyncInfo
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetScope(
                                ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountScope* scope
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebAccountScopeManagerStatics = __uuidof(IWebAccountScopeManagerStatics);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("1e18778b-8805-454b-9f11-468d2af1095a")
                        IWebProviderTokenRequest : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ClientRequest(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenRequest** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccounts(
                                __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_WebAccountSelectionOptions(
                                ABI::Windows::Security::Authentication::Web::Provider::WebAccountSelectionOptions* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationCallbackUri(
                                ABI::Windows::Foundation::IUriRuntimeClass** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetApplicationTokenBindingKeyAsync(
                                ABI::Windows::Security::Authentication::Web::TokenBindingKeyType keyType,
                                ABI::Windows::Foundation::IUriRuntimeClass* target,
                                __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderTokenRequest = __uuidof(IWebProviderTokenRequest);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest2[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("b5d72e4c-10b1-4aa6-88b1-0b6c9e0c1e46")
                        IWebProviderTokenRequest2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetApplicationTokenBindingKeyIdAsync(
                                ABI::Windows::Security::Authentication::Web::TokenBindingKeyType keyType,
                                ABI::Windows::Foundation::IUriRuntimeClass* target,
                                __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** asyncInfo
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderTokenRequest2 = __uuidof(IWebProviderTokenRequest2);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest3[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("1b2716aa-4289-446e-9256-dafb6f66a51e")
                        IWebProviderTokenRequest3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationPackageFamilyName(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ApplicationProcessName(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CheckApplicationForCapabilityAsync(
                                HSTRING capabilityName,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderTokenRequest3 = __uuidof(IWebProviderTokenRequest3);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenResponse[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("ef213793-ef55-4186-b7ce-8cb2e7f9849e")
                        IWebProviderTokenResponse : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ClientResponse(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderTokenResponse = __uuidof(IWebProviderTokenResponse);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenResponseFactory[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Web {
                    namespace Provider {
                        MIDL_INTERFACE("fa49d99a-25ba-4077-9cfa-9db4dea7b71a")
                        IWebProviderTokenResponseFactory : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE Create(
                                ABI::Windows::Security::Authentication::Web::Core::IWebTokenResponse* webTokenResponse,
                                ABI::Windows::Security::Authentication::Web::Provider::IWebProviderTokenResponse** webProviderTokenResponse
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IWebProviderTokenResponseFactory = __uuidof(IWebProviderTokenResponseFactory);
                    } /* Provider */
                } /* Web */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountClientView ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountClientView_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountClientView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountClientView[] = L"Windows.Security.Authentication.Web.Provider.WebAccountClientView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountManager[] = L"Windows.Security.Authentication.Web.Provider.WebAccountManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest[] = L"Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse[] = L"Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3 __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCore_CICryptographicKey** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKeyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
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
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpCookie_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse;

typedef struct __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** items);

    END_INTERFACE
} __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl;

interface __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse
{
    CONST_VTBL struct __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CHttp__CHttpCookie __FIVector_1_Windows__CWeb__CHttp__CHttpCookie;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CHttp__CHttpCookie;

typedef struct __FIVector_1_Windows__CWeb__CHttp__CHttpCookieVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CHttp__CHttpCookie* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpCookie** items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CHttp__CHttpCookieVtbl;

interface __FIVector_1_Windows__CWeb__CHttp__CHttpCookie
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CHttp__CHttpCookieVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CHttp__CHttpCookie_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CHttp__CHttpCookie_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CTokenBindingKeyType __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CTokenBindingKeyType;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountProviderOperationKind __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountProviderOperationKind;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountSelectionOptions __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountSelectionOptions;

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountClientViewType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType
{
    WebAccountClientViewType_IdOnly = 0,
    WebAccountClientViewType_IdAndProperties = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountProviderOperationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountProviderOperationKind
{
    WebAccountProviderOperationKind_RequestToken = 0,
    WebAccountProviderOperationKind_GetTokenSilently = 1,
    WebAccountProviderOperationKind_AddAccount = 2,
    WebAccountProviderOperationKind_ManageAccount = 3,
    WebAccountProviderOperationKind_DeleteAccount = 4,
    WebAccountProviderOperationKind_RetrieveCookies = 5,
    WebAccountProviderOperationKind_SignOutAccount = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope
{
    WebAccountScope_PerUser = 0,
    WebAccountScope_PerApplication = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.Web.Provider.WebAccountSelectionOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountSelectionOptions
{
    WebAccountSelectionOptions_Default = 0,
    WebAccountSelectionOptions_New = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountClientView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountClientView[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountClientView";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationCallbackUri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType* value);
    HRESULT (STDMETHODCALLTYPE* get_AccountPairwiseId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_get_ApplicationCallbackUri(This, value) \
    ((This)->lpVtbl->get_ApplicationCallbackUri(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_get_AccountPairwiseId(This, value) \
    ((This)->lpVtbl->get_AccountPairwiseId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountClientViewFactory[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType viewType,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* applicationCallbackUri,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** view);
    HRESULT (STDMETHODCALLTYPE* CreateWithPairwiseId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountClientViewType viewType,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* applicationCallbackUri,
        HSTRING accountPairwiseId,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView** view);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_Create(This, viewType, applicationCallbackUri, view) \
    ((This)->lpVtbl->Create(This, viewType, applicationCallbackUri, view))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_CreateWithPairwiseId(This, viewType, applicationCallbackUri, accountPairwiseId, view) \
    ((This)->lpVtbl->CreateWithPairwiseId(This, viewType, applicationCallbackUri, accountPairwiseId, view))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientViewFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UpdateWebAccountPropertiesAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* additionalProperties,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* DeleteWebAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* FindAllProviderWebAccountsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* PushCookiesAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIVectorView_1_Windows__CWeb__CHttp__CHttpCookie* cookies,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetViewAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountClientView* view,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* ClearViewAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* applicationCallbackUri,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetViewsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebAccountClientView** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetWebAccountPictureAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* webAccountPicture,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* ClearWebAccountPictureAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_UpdateWebAccountPropertiesAsync(This, webAccount, webAccountUserName, additionalProperties, asyncInfo) \
    ((This)->lpVtbl->UpdateWebAccountPropertiesAsync(This, webAccount, webAccountUserName, additionalProperties, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_AddWebAccountAsync(This, webAccountId, webAccountUserName, props, asyncInfo) \
    ((This)->lpVtbl->AddWebAccountAsync(This, webAccountId, webAccountUserName, props, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_DeleteWebAccountAsync(This, webAccount, asyncInfo) \
    ((This)->lpVtbl->DeleteWebAccountAsync(This, webAccount, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_FindAllProviderWebAccountsAsync(This, asyncInfo) \
    ((This)->lpVtbl->FindAllProviderWebAccountsAsync(This, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_PushCookiesAsync(This, uri, cookies, asyncInfo) \
    ((This)->lpVtbl->PushCookiesAsync(This, uri, cookies, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_SetViewAsync(This, webAccount, view, asyncInfo) \
    ((This)->lpVtbl->SetViewAsync(This, webAccount, view, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_ClearViewAsync(This, webAccount, applicationCallbackUri, asyncInfo) \
    ((This)->lpVtbl->ClearViewAsync(This, webAccount, applicationCallbackUri, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_GetViewsAsync(This, webAccount, asyncInfo) \
    ((This)->lpVtbl->GetViewsAsync(This, webAccount, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_SetWebAccountPictureAsync(This, webAccount, webAccountPicture, asyncInfo) \
    ((This)->lpVtbl->SetWebAccountPictureAsync(This, webAccount, webAccountPicture, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_ClearWebAccountPictureAsync(This, webAccount, asyncInfo) \
    ((This)->lpVtbl->ClearWebAccountPictureAsync(This, webAccount, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics2[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PullCookiesAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2* This,
        HSTRING uriString,
        HSTRING callerPFN,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_PullCookiesAsync(This, uriString, callerPFN, asyncInfo) \
    ((This)->lpVtbl->PullCookiesAsync(This, uriString, callerPFN, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics3[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllProviderWebAccountsForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** operation);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountWithScopeForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope scope,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountWithScopeAndMapForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope scope,
        HSTRING perUserWebAccountId,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_FindAllProviderWebAccountsForUserAsync(This, user, operation) \
    ((This)->lpVtbl->FindAllProviderWebAccountsForUserAsync(This, user, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_AddWebAccountForUserAsync(This, user, webAccountId, webAccountUserName, props, operation) \
    ((This)->lpVtbl->AddWebAccountForUserAsync(This, user, webAccountId, webAccountUserName, props, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_AddWebAccountWithScopeForUserAsync(This, user, webAccountId, webAccountUserName, props, scope, operation) \
    ((This)->lpVtbl->AddWebAccountWithScopeForUserAsync(This, user, webAccountId, webAccountUserName, props, scope, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_AddWebAccountWithScopeAndMapForUserAsync(This, user, webAccountId, webAccountUserName, props, scope, perUserWebAccountId, operation) \
    ((This)->lpVtbl->AddWebAccountWithScopeAndMapForUserAsync(This, user, webAccountId, webAccountUserName, props, scope, perUserWebAccountId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountManagerStatics4[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* InvalidateAppCacheForAllAccountsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* InvalidateAppCacheForAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_InvalidateAppCacheForAllAccountsAsync(This, asyncInfo) \
    ((This)->lpVtbl->InvalidateAppCacheForAllAccountsAsync(This, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_InvalidateAppCacheForAccountAsync(This, webAccount, asyncInfo) \
    ((This)->lpVtbl->InvalidateAppCacheForAccountAsync(This, webAccount, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountMapManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountWithScopeAndMapAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope scope,
        HSTRING perUserWebAccountId,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetPerAppToPerUserAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* perAppAccount,
        HSTRING perUserWebAccountId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetPerUserFromPerAppAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* perAppAccount,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* ClearPerUserFromPerAppAccountAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* perAppAccount,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_AddWebAccountWithScopeAndMapAsync(This, webAccountId, webAccountUserName, props, scope, perUserWebAccountId, asyncInfo) \
    ((This)->lpVtbl->AddWebAccountWithScopeAndMapAsync(This, webAccountId, webAccountUserName, props, scope, perUserWebAccountId, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_SetPerAppToPerUserAccountAsync(This, perAppAccount, perUserWebAccountId, asyncInfo) \
    ((This)->lpVtbl->SetPerAppToPerUserAccountAsync(This, perAppAccount, perUserWebAccountId, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_GetPerUserFromPerAppAccountAsync(This, perAppAccount, asyncInfo) \
    ((This)->lpVtbl->GetPerUserFromPerAppAccountAsync(This, perAppAccount, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_ClearPerUserFromPerAppAccountAsync(This, perAppAccount, asyncInfo) \
    ((This)->lpVtbl->ClearPerUserFromPerAppAccountAsync(This, perAppAccount, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountMapManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderAddAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderAddAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderBaseReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This);
    HRESULT (STDMETHODCALLTYPE* ReportError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_ReportError(This, value) \
    ((This)->lpVtbl->ReportError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderBaseReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderDeleteAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderDeleteAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderManageAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderManageAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountProviderOperationKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderRetrieveCookiesOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** webCookieRequestContext);
    HRESULT (STDMETHODCALLTYPE* get_Cookies)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        __FIVector_1_Windows__CWeb__CHttp__CHttpCookie** cookies);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** uri);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationCallbackUri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_get_Context(This, webCookieRequestContext) \
    ((This)->lpVtbl->get_Context(This, webCookieRequestContext))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_get_Cookies(This, cookies) \
    ((This)->lpVtbl->get_Cookies(This, cookies))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_put_Uri(This, uri) \
    ((This)->lpVtbl->put_Uri(This, uri))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_get_Uri(This, uri) \
    ((This)->lpVtbl->get_Uri(This, uri))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_get_ApplicationCallbackUri(This, value) \
    ((This)->lpVtbl->get_ApplicationCallbackUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderRetrieveCookiesOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderSignOutAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationCallbackUri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ClientId)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_get_ApplicationCallbackUri(This, value) \
    ((This)->lpVtbl->get_ApplicationCallbackUri(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_get_ClientId(This, value) \
    ((This)->lpVtbl->get_ClientId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSignOutAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderSilentReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportUserInteractionRequired)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This);
    HRESULT (STDMETHODCALLTYPE* ReportUserInteractionRequiredWithError)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebProviderError* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_ReportUserInteractionRequired(This) \
    ((This)->lpVtbl->ReportUserInteractionRequired(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_ReportUserInteractionRequiredWithError(This, value) \
    ((This)->lpVtbl->ReportUserInteractionRequiredWithError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderSilentReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenObjects[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjectsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjectsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjectsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenObjects2[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenObjects2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderTokenOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderRequest)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest** webTokenRequest);
    HRESULT (STDMETHODCALLTYPE* get_ProviderResponses)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        __FIVector_1_Windows__CSecurity__CAuthentication__CWeb__CProvider__CWebProviderTokenResponse** value);
    HRESULT (STDMETHODCALLTYPE* put_CacheExpirationTime)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_CacheExpirationTime)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_get_ProviderRequest(This, webTokenRequest) \
    ((This)->lpVtbl->get_ProviderRequest(This, webTokenRequest))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_get_ProviderResponses(This, value) \
    ((This)->lpVtbl->get_ProviderResponses(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_put_CacheExpirationTime(This, value) \
    ((This)->lpVtbl->put_CacheExpirationTime(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_get_CacheExpirationTime(This, value) \
    ((This)->lpVtbl->get_CacheExpirationTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderTokenOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountProviderUIReportOperation[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportUserCanceled)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_ReportUserCanceled(This) \
    ((This)->lpVtbl->ReportUserCanceled(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountProviderUIReportOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebAccountScopeManagerStatics[] = L"Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddWebAccountWithScopeAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        HSTRING webAccountId,
        HSTRING webAccountUserName,
        __FIMapView_2_HSTRING_HSTRING* props,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope scope,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CWebAccount** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SetScopeAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope scope,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* GetScope)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountScope* scope);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_AddWebAccountWithScopeAsync(This, webAccountId, webAccountUserName, props, scope, asyncInfo) \
    ((This)->lpVtbl->AddWebAccountWithScopeAsync(This, webAccountId, webAccountUserName, props, scope, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_SetScopeAsync(This, webAccount, scope, asyncInfo) \
    ((This)->lpVtbl->SetScopeAsync(This, webAccount, scope, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_GetScope(This, webAccount, scope) \
    ((This)->lpVtbl->GetScope(This, webAccount, scope))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebAccountScopeManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClientRequest)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenRequest** value);
    HRESULT (STDMETHODCALLTYPE* get_WebAccounts)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountSelectionOptions)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CWebAccountSelectionOptions* value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationCallbackUri)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* GetApplicationTokenBindingKeyAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CTokenBindingKeyType keyType,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* target,
        __FIAsyncOperation_1_Windows__CSecurity__CCryptography__CCore__CCryptographicKey** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequestVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_get_ClientRequest(This, value) \
    ((This)->lpVtbl->get_ClientRequest(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_get_WebAccounts(This, value) \
    ((This)->lpVtbl->get_WebAccounts(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_get_WebAccountSelectionOptions(This, value) \
    ((This)->lpVtbl->get_WebAccountSelectionOptions(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_get_ApplicationCallbackUri(This, value) \
    ((This)->lpVtbl->get_ApplicationCallbackUri(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_GetApplicationTokenBindingKeyAsync(This, keyType, target, asyncInfo) \
    ((This)->lpVtbl->GetApplicationTokenBindingKeyAsync(This, keyType, target, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest2[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetApplicationTokenBindingKeyIdAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CTokenBindingKeyType keyType,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* target,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIBuffer** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_GetApplicationTokenBindingKeyIdAsync(This, keyType, target, asyncInfo) \
    ((This)->lpVtbl->GetApplicationTokenBindingKeyIdAsync(This, keyType, target, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenRequest3[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationPackageFamilyName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationProcessName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* CheckApplicationForCapabilityAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3* This,
        HSTRING capabilityName,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_get_ApplicationPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_ApplicationPackageFamilyName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_get_ApplicationProcessName(This, value) \
    ((This)->lpVtbl->get_ApplicationProcessName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_CheckApplicationForCapabilityAsync(This, capabilityName, operation) \
    ((This)->lpVtbl->CheckApplicationForCapabilityAsync(This, capabilityName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenResponse[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ClientResponse)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_get_ClientResponse(This, value) \
    ((This)->lpVtbl->get_ClientResponse(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Web_Provider_IWebProviderTokenResponseFactory[] = L"Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CCore_CIWebTokenResponse* webTokenResponse,
        __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponse** webProviderTokenResponse);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_Create(This, webTokenResponse, webProviderTokenResponse) \
    ((This)->lpVtbl->Create(This, webTokenResponse, webProviderTokenResponse))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CWeb_CProvider_CIWebProviderTokenResponseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountClientView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Provider.IWebAccountClientViewFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountClientView ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountClientView_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountClientView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountClientView[] = L"Windows.Security.Authentication.Web.Provider.WebAccountClientView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics2 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountMapManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountManagerStatics4 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Web.Provider.IWebAccountScopeManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountManager[] = L"Windows.Security.Authentication.Web.Provider.WebAccountManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderAddAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderAddAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderDeleteAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderDeleteAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderSilentReportOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderGetTokenSilentOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderManageAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderManageAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderUIReportOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderRequestTokenOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderRetrieveCookiesOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderRetrieveCookiesOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderSignOutAccountOperation ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderOperation
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderBaseReportOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderSignOutAccountOperation[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebAccountProviderTokenObjects2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebAccountProviderTriggerDetails[] = L"Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest ** Default Interface **
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest2
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenRequest3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebProviderTokenRequest[] = L"Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponseFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Web.Provider.IWebProviderTokenResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Web_Provider_WebProviderTokenResponse[] = L"Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse";
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
#endif // __windows2Esecurity2Eauthentication2Eweb2Eprovider_p_h__

#endif // __windows2Esecurity2Eauthentication2Eweb2Eprovider_h__
