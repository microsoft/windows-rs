
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
#ifndef __windows2Esecurity2Ecredentials_h__
#define __windows2Esecurity2Ecredentials_h__
#ifndef __windows2Esecurity2Ecredentials_p_h__
#define __windows2Esecurity2Ecredentials_p_h__


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
#include "Windows.Security.Cryptography.Core.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IAttestationChallengeHandler;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler ABI::Windows::Security::Credentials::IAttestationChallengeHandler

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface ICredentialFactory;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory ABI::Windows::Security::Credentials::ICredentialFactory

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential ABI::Windows::Security::Credentials::IKeyCredential

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredential2;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2 ABI::Windows::Security::Credentials::IKeyCredential2

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialAttestationResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult ABI::Windows::Security::Credentials::IKeyCredentialAttestationResult

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialCacheConfiguration;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration ABI::Windows::Security::Credentials::IKeyCredentialCacheConfiguration

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialCacheConfigurationFactory;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory ABI::Windows::Security::Credentials::IKeyCredentialCacheConfigurationFactory

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialManagerCreateWithWindowStatics;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics ABI::Windows::Security::Credentials::IKeyCredentialManagerCreateWithWindowStatics

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialManagerStatics;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics ABI::Windows::Security::Credentials::IKeyCredentialManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialManagerStatics2;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2 ABI::Windows::Security::Credentials::IKeyCredentialManagerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialOperationResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult ABI::Windows::Security::Credentials::IKeyCredentialOperationResult

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialRetrievalResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult ABI::Windows::Security::Credentials::IKeyCredentialRetrievalResult

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IKeyCredentialWithWindow;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow ABI::Windows::Security::Credentials::IKeyCredentialWithWindow

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IPasswordVault;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault ABI::Windows::Security::Credentials::IPasswordVault

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccount2;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2 ABI::Windows::Security::Credentials::IWebAccount2

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountFactory;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory ABI::Windows::Security::Credentials::IWebAccountFactory

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProvider2;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2 ABI::Windows::Security::Credentials::IWebAccountProvider2

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProvider3;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3 ABI::Windows::Security::Credentials::IWebAccountProvider3

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProvider4;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4 ABI::Windows::Security::Credentials::IWebAccountProvider4

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProviderFactory;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory ABI::Windows::Security::Credentials::IWebAccountProviderFactory

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__

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
            namespace Credentials {
                class KeyCredentialAttestationResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b83d29e9-f4e4-5aa4-92d5-b262cb40c622"))
IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*, ABI::Windows::Security::Credentials::IKeyCredentialAttestationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.KeyCredentialAttestationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c16e103-f783-5dd9-a5f3-3362bcbdaabd"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*, ABI::Windows::Security::Credentials::IKeyCredentialAttestationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.KeyCredentialAttestationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialAttestationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class KeyCredentialOperationResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6576e5b3-9535-50d6-98f6-c67d6aaca2c5"))
IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*, ABI::Windows::Security::Credentials::IKeyCredentialOperationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.KeyCredentialOperationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("39b4609a-0202-55fa-8005-6f83709e20f3"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*, ABI::Windows::Security::Credentials::IKeyCredentialOperationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.KeyCredentialOperationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialOperationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class KeyCredentialRetrievalResult;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89d0ad1e-bd4c-55b4-810e-bddd4cec7a2a"))
IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*, ABI::Windows::Security::Credentials::IKeyCredentialRetrievalResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.KeyCredentialRetrievalResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03ea60b1-a874-58ce-8e8e-fff448b6733e"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*, ABI::Windows::Security::Credentials::IKeyCredentialRetrievalResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.KeyCredentialRetrievalResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::KeyCredentialRetrievalResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

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
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b01093d8-4f52-50f0-9aa4-e22639111162"))
IIterator<ABI::Windows::Security::Credentials::PasswordCredential*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::PasswordCredential*, ABI::Windows::Security::Credentials::IPasswordCredential*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Credentials.PasswordCredential>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Credentials::PasswordCredential*> __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_t;
#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d224a66-bad5-5ad5-9ade-1e9f5a60fe73"))
IIterable<ABI::Windows::Security::Credentials::PasswordCredential*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::PasswordCredential*, ABI::Windows::Security::Credentials::IPasswordCredential*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Credentials.PasswordCredential>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Credentials::PasswordCredential*> __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_t;
#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapChangedEventArgs_1_HSTRING_USE
#define DEF___FIMapChangedEventArgs_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60141efb-f2f9-5377-96fd-f8c60d9558b5"))
IMapChangedEventArgs<HSTRING> : IMapChangedEventArgs_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapChangedEventArgs`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapChangedEventArgs<HSTRING> __FIMapChangedEventArgs_1_HSTRING_t;
#define __FIMapChangedEventArgs_1_HSTRING ABI::Windows::Foundation::Collections::__FIMapChangedEventArgs_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapChangedEventArgs_1_HSTRING_USE */



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



#ifndef DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#define DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("24f981e5-ddca-538d-aada-a59906084cf1"))
MapChangedEventHandler<HSTRING, IInspectable*> : MapChangedEventHandler_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.MapChangedEventHandler`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef MapChangedEventHandler<HSTRING, IInspectable*> __FMapChangedEventHandler_2_HSTRING_IInspectable_t;
#define __FMapChangedEventHandler_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FMapChangedEventHandler_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FMapChangedEventHandler_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#define DEF___FIObservableMap_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("236aac9d-fb12-5c4d-a41c-9e445fb4d7ec"))
IObservableMap<HSTRING, IInspectable*> : IObservableMap_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IObservableMap`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IObservableMap<HSTRING, IInspectable*> __FIObservableMap_2_HSTRING_IInspectable_t;
#define __FIObservableMap_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIObservableMap_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIObservableMap_2_HSTRING_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e6185ad-a6df-5428-aff5-17e45f3f476f"))
IVectorView<ABI::Windows::Security::Credentials::PasswordCredential*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::PasswordCredential*, ABI::Windows::Security::Credentials::IPasswordCredential*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Credentials.PasswordCredential>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Credentials::PasswordCredential*> __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_t;
#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                typedef enum CollectionChange : int CollectionChange;
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
            typedef struct TimeSpan TimeSpan;
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
            namespace Cryptography {
                namespace Core {
                    typedef enum CryptographicPublicKeyBlobType : int CryptographicPublicKeyBlobType;
                } /* Core */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

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
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum ChallengeResponseKind : int ChallengeResponseKind;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum KeyCredentialAttestationStatus : int KeyCredentialAttestationStatus;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum KeyCredentialCacheOption : int KeyCredentialCacheOption;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum KeyCredentialCreationOption : int KeyCredentialCreationOption;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum KeyCredentialStatus : int KeyCredentialStatus;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum WebAccountPictureSize : int WebAccountPictureSize;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                typedef enum WebAccountState : int WebAccountState;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class KeyCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class KeyCredentialCacheConfiguration;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccountProvider;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Credentials.ChallengeResponseKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum ChallengeResponseKind : int
                {
                    ChallengeResponseKind_VirtualizationBasedSecurityEnclave = 0,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialAttestationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum KeyCredentialAttestationStatus : int
                {
                    KeyCredentialAttestationStatus_Success = 0,
                    KeyCredentialAttestationStatus_UnknownError = 1,
                    KeyCredentialAttestationStatus_NotSupported = 2,
                    KeyCredentialAttestationStatus_TemporaryFailure = 3,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialCacheOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum KeyCredentialCacheOption : int
                {
                    KeyCredentialCacheOption_NoCache = 0,
                    KeyCredentialCacheOption_CacheWhenUnlocked = 1,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialCreationOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum KeyCredentialCreationOption : int
                {
                    KeyCredentialCreationOption_ReplaceExisting = 0,
                    KeyCredentialCreationOption_FailIfExists = 1,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum KeyCredentialStatus : int
                {
                    KeyCredentialStatus_Success = 0,
                    KeyCredentialStatus_UnknownError = 1,
                    KeyCredentialStatus_NotFound = 2,
                    KeyCredentialStatus_UserCanceled = 3,
                    KeyCredentialStatus_UserPrefersPassword = 4,
                    KeyCredentialStatus_CredentialAlreadyExists = 5,
                    KeyCredentialStatus_SecurityDeviceLocked = 6,
                    KeyCredentialStatus_AlgorithmNotSupported = 7,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.WebAccountPictureSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum WebAccountPictureSize : int
                {
                    WebAccountPictureSize_Size64x64 = 64,
                    WebAccountPictureSize_Size208x208 = 208,
                    WebAccountPictureSize_Size424x424 = 424,
                    WebAccountPictureSize_Size1080x1080 = 1080,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.WebAccountState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                enum WebAccountState : int
                {
                    WebAccountState_None = 0,
                    WebAccountState_Connected = 1,
                    WebAccountState_Error = 2,
                };
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Security.Credentials.AttestationChallengeHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("f6ae35b0-d805-587d-944f-a09bd032acf5")
                IAttestationChallengeHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Storage::Streams::IBuffer* challenge,
                        ABI::Windows::Storage::Streams::IBuffer** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAttestationChallengeHandler = __uuidof(IAttestationChallengeHandler);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.ICredentialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_ICredentialFactory[] = L"Windows.Security.Credentials.ICredentialFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("54ef13a1-bf26-47b5-97dd-de779b7cad58")
                ICredentialFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePasswordCredential(
                        HSTRING resource,
                        HSTRING userName,
                        HSTRING password,
                        ABI::Windows::Security::Credentials::IPasswordCredential** credential
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICredentialFactory = __uuidof(ICredentialFactory);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredential[] = L"Windows.Security.Credentials.IKeyCredential";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("9585ef8d-457b-4847-b11a-fa960bbdb138")
                IKeyCredential : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrievePublicKeyWithDefaultBlobType(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrievePublicKeyWithBlobType(
                        ABI::Windows::Security::Cryptography::Core::CryptographicPublicKeyBlobType blobType,
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestSignAsync(
                        ABI::Windows::Storage::Streams::IBuffer* data,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAttestationAsync(
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredential = __uuidof(IKeyCredential);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredential2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredential2[] = L"Windows.Security.Credentials.IKeyCredential2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("3b7c5e09-7b72-5a05-b2f0-7119ca3fd5df")
                IKeyCredential2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestDeriveSharedSecretAsync(
                        ABI::Windows::UI::WindowId windowId,
                        HSTRING message,
                        ABI::Windows::Storage::Streams::IBuffer* encryptedRequest,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrieveAuthorizationContext(
                        ABI::Windows::Storage::Streams::IBuffer* encryptedRequest,
                        ABI::Windows::Storage::Streams::IBuffer** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredential2 = __uuidof(IKeyCredential2);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialAttestationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialAttestationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialAttestationResult[] = L"Windows.Security.Credentials.IKeyCredentialAttestationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("78aab3a1-a3c1-4103-b6cc-472c44171cbb")
                IKeyCredentialAttestationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CertificateChainBuffer(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttestationBuffer(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Credentials::KeyCredentialAttestationStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialAttestationResult = __uuidof(IKeyCredentialAttestationResult);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialCacheConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialCacheConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialCacheConfiguration[] = L"Windows.Security.Credentials.IKeyCredentialCacheConfiguration";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("438bd21a-61ff-5468-95a6-b1d5216e458d")
                IKeyCredentialCacheConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CacheOption(
                        ABI::Windows::Security::Credentials::KeyCredentialCacheOption* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UsageCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialCacheConfiguration = __uuidof(IKeyCredentialCacheConfiguration);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialCacheConfigurationFactory[] = L"Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("9948c31b-c827-5b58-9442-40acd8ab1e7d")
                IKeyCredentialCacheConfigurationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        ABI::Windows::Security::Credentials::KeyCredentialCacheOption cacheOption,
                        ABI::Windows::Foundation::TimeSpan timeout,
                        UINT32 usageCount,
                        ABI::Windows::Security::Credentials::IKeyCredentialCacheConfiguration** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialCacheConfigurationFactory = __uuidof(IKeyCredentialCacheConfigurationFactory);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerCreateWithWindowStatics[] = L"Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("30b1b9c9-61ef-43e8-88ac-cc433b38d1a6")
                IKeyCredentialManagerCreateWithWindowStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateForWindowAsync(
                        ABI::Windows::UI::WindowId window,
                        HSTRING name,
                        ABI::Windows::Security::Credentials::KeyCredentialCreationOption option,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialManagerCreateWithWindowStatics = __uuidof(IKeyCredentialManagerCreateWithWindowStatics);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerStatics[] = L"Windows.Security.Credentials.IKeyCredentialManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("6aac468b-0ef1-4ce0-8290-4106da6a63b5")
                IKeyCredentialManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsSupportedAsync(
                        __FIAsyncOperation_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RenewAttestationAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsync(
                        HSTRING name,
                        ABI::Windows::Security::Credentials::KeyCredentialCreationOption option,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                        HSTRING name,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        HSTRING name,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialManagerStatics = __uuidof(IKeyCredentialManagerStatics);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IKeyCredentialManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerStatics2[] = L"Windows.Security.Credentials.IKeyCredentialManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("6439895d-68c5-521b-9dc4-7c199794f0d8")
                IKeyCredentialManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestCreateAsync(
                        HSTRING name,
                        ABI::Windows::Security::Credentials::KeyCredentialCreationOption option,
                        HSTRING algorithm,
                        HSTRING message,
                        ABI::Windows::Security::Credentials::IKeyCredentialCacheConfiguration* cacheConfiguration,
                        ABI::Windows::UI::WindowId windowId,
                        ABI::Windows::Security::Credentials::ChallengeResponseKind callbackType,
                        ABI::Windows::Security::Credentials::IAttestationChallengeHandler* attestationCallback,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                        HSTRING name,
                        ABI::Windows::Security::Credentials::ChallengeResponseKind callbackType,
                        ABI::Windows::Security::Credentials::IAttestationChallengeHandler* attestationCallback,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSecureId(
                        ABI::Windows::Storage::Streams::IBuffer** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialManagerStatics2 = __uuidof(IKeyCredentialManagerStatics2);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialOperationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialOperationResult[] = L"Windows.Security.Credentials.IKeyCredentialOperationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("f53786c1-5261-4cdd-976d-cc909ac71620")
                IKeyCredentialOperationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Credentials::KeyCredentialStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialOperationResult = __uuidof(IKeyCredentialOperationResult);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialRetrievalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialRetrievalResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialRetrievalResult[] = L"Windows.Security.Credentials.IKeyCredentialRetrievalResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("58cd7703-8d87-4249-9b58-f6598cc9644e")
                IKeyCredentialRetrievalResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Credential(
                        ABI::Windows::Security::Credentials::IKeyCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Credentials::KeyCredentialStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialRetrievalResult = __uuidof(IKeyCredentialRetrievalResult);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialWithWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialWithWindow[] = L"Windows.Security.Credentials.IKeyCredentialWithWindow";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("050deb3b-b19c-4635-9df6-5650d66c62b1")
                IKeyCredentialWithWindow : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestSignForWindowAsync(
                        ABI::Windows::UI::WindowId window,
                        ABI::Windows::Storage::Streams::IBuffer* data,
                        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyCredentialWithWindow = __uuidof(IKeyCredentialWithWindow);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IPasswordCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IPasswordCredential[] = L"Windows.Security.Credentials.IPasswordCredential";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("6ab18989-c720-41a7-a6c1-feadb36329a0")
                IPasswordCredential : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Resource(
                        HSTRING* resource
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Resource(
                        HSTRING resource
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserName(
                        HSTRING* userName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UserName(
                        HSTRING userName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Password(
                        HSTRING* password
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Password(
                        HSTRING password
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrievePassword(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** props
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPasswordCredential = __uuidof(IPasswordCredential);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IPasswordVault
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordVault
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IPasswordVault[] = L"Windows.Security.Credentials.IPasswordVault";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("61fd2c0b-c8d4-48c1-a54f-bc5a64205af2")
                IPasswordVault : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Add(
                        ABI::Windows::Security::Credentials::IPasswordCredential* credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Remove(
                        ABI::Windows::Security::Credentials::IPasswordCredential* credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Retrieve(
                        HSTRING resource,
                        HSTRING userName,
                        ABI::Windows::Security::Credentials::IPasswordCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllByResource(
                        HSTRING resource,
                        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllByUserName(
                        HSTRING userName,
                        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrieveAll(
                        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPasswordVault = __uuidof(IPasswordVault);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccount[] = L"Windows.Security.Credentials.IWebAccount";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("69473eb2-8031-49be-80bb-96cb46d99aba")
                IWebAccount : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebAccountProvider(
                        ABI::Windows::Security::Credentials::IWebAccountProvider** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Security::Credentials::WebAccountState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccount = __uuidof(IWebAccount);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccount2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccount
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccount2[] = L"Windows.Security.Credentials.IWebAccount2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("7b56d6f8-990b-4eb5-94a7-5621f3a8b824")
                IWebAccount2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPictureAsync(
                        ABI::Windows::Security::Credentials::WebAccountPictureSize desizedSize,
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SignOutAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SignOutWithClientIdAsync(
                        HSTRING clientId,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccount2 = __uuidof(IWebAccount2);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountFactory[] = L"Windows.Security.Credentials.IWebAccountFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("ac9afb39-1de9-4e92-b78f-0581a87f6e5c")
                IWebAccountFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWebAccount(
                        ABI::Windows::Security::Credentials::IWebAccountProvider* webAccountProvider,
                        HSTRING userName,
                        ABI::Windows::Security::Credentials::WebAccountState state,
                        ABI::Windows::Security::Credentials::IWebAccount** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountFactory = __uuidof(IWebAccountFactory);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider[] = L"Windows.Security.Credentials.IWebAccountProvider";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("29dcc8c3-7ab9-4a7c-a336-b942f9dbf7c7")
                IWebAccountProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("IconUri may be altered or unavailable for releases after Windows 8.2. Instead, use Icon.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_IconUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProvider = __uuidof(IWebAccountProvider);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider2[] = L"Windows.Security.Credentials.IWebAccountProvider2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("4a01eb05-4e42-41d4-b518-e008a5163614")
                IWebAccountProvider2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayPurpose(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Authority(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProvider2 = __uuidof(IWebAccountProvider2);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccountProvider2
 *     Windows.Security.Credentials.IWebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider3[] = L"Windows.Security.Credentials.IWebAccountProvider3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("da1c518b-970d-4d49-825c-f2706f8ca7fe")
                IWebAccountProvider3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** user
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProvider3 = __uuidof(IWebAccountProvider3);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider4[] = L"Windows.Security.Credentials.IWebAccountProvider4";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("718fd8db-e796-4210-b74e-84d29894b080")
                IWebAccountProvider4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSystemProvider(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProvider4 = __uuidof(IWebAccountProvider4);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProviderFactory[] = L"Windows.Security.Credentials.IWebAccountProviderFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                MIDL_INTERFACE("1d767df1-e1e1-4b9a-a774-5c7c7e3bf371")
                IWebAccountProviderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWebAccountProvider(
                        HSTRING id,
                        HSTRING displayName,
                        ABI::Windows::Foundation::IUriRuntimeClass* iconUri,
                        ABI::Windows::Security::Credentials::IWebAccountProvider** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProviderFactory = __uuidof(IWebAccountProviderFactory);
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredential ** Default Interface **
 *    Windows.Security.Credentials.IKeyCredential2
 *    Windows.Security.Credentials.IKeyCredentialWithWindow
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredential_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredential[] = L"Windows.Security.Credentials.KeyCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialAttestationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialAttestationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialAttestationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialAttestationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialAttestationResult[] = L"Windows.Security.Credentials.KeyCredentialAttestationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialCacheConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialCacheConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialCacheConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialCacheConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialCacheConfiguration[] = L"Windows.Security.Credentials.KeyCredentialCacheConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerStatics2 interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialManager[] = L"Windows.Security.Credentials.KeyCredentialManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialOperationResult[] = L"Windows.Security.Credentials.KeyCredentialOperationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialRetrievalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialRetrievalResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialRetrievalResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialRetrievalResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialRetrievalResult[] = L"Windows.Security.Credentials.KeyCredentialRetrievalResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.ICredentialFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IPasswordCredential ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordCredential_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordCredential[] = L"Windows.Security.Credentials.PasswordCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordCredentialPropertyStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordCredentialPropertyStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordCredentialPropertyStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordCredentialPropertyStore[] = L"Windows.Security.Credentials.PasswordCredentialPropertyStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordVault
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IPasswordVault ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordVault_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordVault_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordVault[] = L"Windows.Security.Credentials.PasswordVault";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.WebAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IWebAccountFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IWebAccount ** Default Interface **
 *    Windows.Security.Credentials.IWebAccount2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_WebAccount_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_WebAccount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_WebAccount[] = L"Windows.Security.Credentials.WebAccount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.WebAccountProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IWebAccountProviderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IWebAccountProvider ** Default Interface **
 *    Windows.Security.Credentials.IWebAccountProvider2
 *    Windows.Security.Credentials.IWebAccountProvider3
 *    Windows.Security.Credentials.IWebAccountProvider4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_WebAccountProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_WebAccountProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_WebAccountProvider[] = L"Windows.Security.Credentials.WebAccountProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2 __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2 __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2 __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2 __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3 __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4 __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential;

typedef struct __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl;

interface __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential;

typedef struct __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        __FIIterator_1_Windows__CSecurity__CCredentials__CPasswordCredential** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl;

interface __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapChangedEventArgs_1_HSTRING __FIMapChangedEventArgs_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapChangedEventArgs_1_HSTRING;

typedef struct __FIMapChangedEventArgs_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapChangedEventArgs_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapChangedEventArgs_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapChangedEventArgs_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapChangedEventArgs_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapChangedEventArgs_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionChange)(__FIMapChangedEventArgs_1_HSTRING* This,
        enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange* result);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIMapChangedEventArgs_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIMapChangedEventArgs_1_HSTRINGVtbl;

interface __FIMapChangedEventArgs_1_HSTRING
{
    CONST_VTBL struct __FIMapChangedEventArgs_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapChangedEventArgs_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapChangedEventArgs_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapChangedEventArgs_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapChangedEventArgs_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapChangedEventArgs_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapChangedEventArgs_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapChangedEventArgs_1_HSTRING_get_CollectionChange(This, result) \
    ((This)->lpVtbl->get_CollectionChange(This, result))

#define __FIMapChangedEventArgs_1_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#endif /* COBJMACROS */

#endif // ____FIMapChangedEventArgs_1_HSTRING_INTERFACE_DEFINED__

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

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

#if !defined(____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FMapChangedEventHandler_2_HSTRING_IInspectable __FMapChangedEventHandler_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FMapChangedEventHandler_2_HSTRING_IInspectable;

typedef struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FMapChangedEventHandler_2_HSTRING_IInspectable* This,
        __FIObservableMap_2_HSTRING_IInspectable* sender,
        __FIMapChangedEventArgs_1_HSTRING* event);

    END_INTERFACE
} __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl;

interface __FMapChangedEventHandler_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FMapChangedEventHandler_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FMapChangedEventHandler_2_HSTRING_IInspectable_Invoke(This, sender, event) \
    ((This)->lpVtbl->Invoke(This, sender, event))

#endif /* COBJMACROS */

#endif // ____FMapChangedEventHandler_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIObservableMap_2_HSTRING_IInspectable __FIObservableMap_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIObservableMap_2_HSTRING_IInspectable;

typedef struct __FIObservableMap_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIObservableMap_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIObservableMap_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIObservableMap_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIObservableMap_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIObservableMap_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIObservableMap_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        __FMapChangedEventHandler_2_HSTRING_IInspectable* vhnd,
        EventRegistrationToken* result);
    HRESULT (STDMETHODCALLTYPE* remove_MapChanged)(__FIObservableMap_2_HSTRING_IInspectable* This,
        EventRegistrationToken token);

    END_INTERFACE
} __FIObservableMap_2_HSTRING_IInspectableVtbl;

interface __FIObservableMap_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIObservableMap_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIObservableMap_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIObservableMap_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIObservableMap_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIObservableMap_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIObservableMap_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIObservableMap_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIObservableMap_2_HSTRING_IInspectable_add_MapChanged(This, vhnd, result) \
    ((This)->lpVtbl->add_MapChanged(This, vhnd, result))

#define __FIObservableMap_2_HSTRING_IInspectable_remove_MapChanged(This, token) \
    ((This)->lpVtbl->remove_MapChanged(This, token))

#endif /* COBJMACROS */

#endif // ____FIObservableMap_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential;

typedef struct __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange __x_ABI_CWindows_CFoundation_CCollections_CCollectionChange;

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CChallengeResponseKind __x_ABI_CWindows_CSecurity_CCredentials_CChallengeResponseKind;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialAttestationStatus __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialAttestationStatus;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCacheOption __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCacheOption;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialStatus __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialStatus;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountPictureSize __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountPictureSize;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountState __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountState;

/*
 *
 * Struct Windows.Security.Credentials.ChallengeResponseKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CSecurity_CCredentials_CChallengeResponseKind
{
    ChallengeResponseKind_VirtualizationBasedSecurityEnclave = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialAttestationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialAttestationStatus
{
    KeyCredentialAttestationStatus_Success = 0,
    KeyCredentialAttestationStatus_UnknownError = 1,
    KeyCredentialAttestationStatus_NotSupported = 2,
    KeyCredentialAttestationStatus_TemporaryFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialCacheOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCacheOption
{
    KeyCredentialCacheOption_NoCache = 0,
    KeyCredentialCacheOption_CacheWhenUnlocked = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialCreationOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption
{
    KeyCredentialCreationOption_ReplaceExisting = 0,
    KeyCredentialCreationOption_FailIfExists = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.KeyCredentialStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialStatus
{
    KeyCredentialStatus_Success = 0,
    KeyCredentialStatus_UnknownError = 1,
    KeyCredentialStatus_NotFound = 2,
    KeyCredentialStatus_UserCanceled = 3,
    KeyCredentialStatus_UserPrefersPassword = 4,
    KeyCredentialStatus_CredentialAlreadyExists = 5,
    KeyCredentialStatus_SecurityDeviceLocked = 6,
    KeyCredentialStatus_AlgorithmNotSupported = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.WebAccountPictureSize
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountPictureSize
{
    WebAccountPictureSize_Size64x64 = 64,
    WebAccountPictureSize_Size208x208 = 208,
    WebAccountPictureSize_Size424x424 = 424,
    WebAccountPictureSize_Size1080x1080 = 1080,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.WebAccountState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountState
{
    WebAccountState_None = 0,
    WebAccountState_Connected = 1,
    WebAccountState_Error = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Security.Credentials.AttestationChallengeHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* challenge,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandlerVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_Invoke(This, challenge, result) \
    ((This)->lpVtbl->Invoke(This, challenge, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.ICredentialFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_ICredentialFactory[] = L"Windows.Security.Credentials.ICredentialFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePasswordCredential)(__x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory* This,
        HSTRING resource,
        HSTRING userName,
        HSTRING password,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** credential);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_CreatePasswordCredential(This, resource, userName, password, credential) \
    ((This)->lpVtbl->CreatePasswordCredential(This, resource, userName, password, credential))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CICredentialFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredential[] = L"Windows.Security.Credentials.IKeyCredential";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* RetrievePublicKeyWithDefaultBlobType)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* RetrievePublicKeyWithBlobType)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        enum __x_ABI_CWindows_CSecurity_CCryptography_CCore_CCryptographicPublicKeyBlobType blobType,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* RequestSignAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** value);
    HRESULT (STDMETHODCALLTYPE* GetAttestationAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialAttestationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_RetrievePublicKeyWithDefaultBlobType(This, value) \
    ((This)->lpVtbl->RetrievePublicKeyWithDefaultBlobType(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_RetrievePublicKeyWithBlobType(This, blobType, value) \
    ((This)->lpVtbl->RetrievePublicKeyWithBlobType(This, blobType, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_RequestSignAsync(This, data, value) \
    ((This)->lpVtbl->RequestSignAsync(This, data, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_GetAttestationAsync(This, value) \
    ((This)->lpVtbl->GetAttestationAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredential2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredential2[] = L"Windows.Security.Credentials.IKeyCredential2";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestDeriveSharedSecretAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        HSTRING message,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* encryptedRequest,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** operation);
    HRESULT (STDMETHODCALLTYPE* RetrieveAuthorizationContext)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* encryptedRequest,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_RequestDeriveSharedSecretAsync(This, windowId, message, encryptedRequest, operation) \
    ((This)->lpVtbl->RequestDeriveSharedSecretAsync(This, windowId, message, encryptedRequest, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_RetrieveAuthorizationContext(This, encryptedRequest, result) \
    ((This)->lpVtbl->RetrieveAuthorizationContext(This, encryptedRequest, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialAttestationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialAttestationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialAttestationResult[] = L"Windows.Security.Credentials.IKeyCredentialAttestationResult";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CertificateChainBuffer)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_AttestationBuffer)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialAttestationStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_get_CertificateChainBuffer(This, value) \
    ((This)->lpVtbl->get_CertificateChainBuffer(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_get_AttestationBuffer(This, value) \
    ((This)->lpVtbl->get_AttestationBuffer(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialAttestationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialCacheConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialCacheConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialCacheConfiguration[] = L"Windows.Security.Credentials.IKeyCredentialCacheConfiguration";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CacheOption)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCacheOption* value);
    HRESULT (STDMETHODCALLTYPE* get_Timeout)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UsageCount)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_get_CacheOption(This, value) \
    ((This)->lpVtbl->get_CacheOption(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_get_Timeout(This, value) \
    ((This)->lpVtbl->get_Timeout(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_get_UsageCount(This, value) \
    ((This)->lpVtbl->get_UsageCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialCacheConfigurationFactory[] = L"Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCacheOption cacheOption,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan timeout,
        UINT32 usageCount,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_CreateInstance(This, cacheOption, timeout, usageCount, result) \
    ((This)->lpVtbl->CreateInstance(This, cacheOption, timeout, usageCount, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfigurationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerCreateWithWindowStatics[] = L"Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestCreateForWindowAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics* This,
        struct __x_ABI_CWindows_CUI_CWindowId window,
        HSTRING name,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption option,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_RequestCreateForWindowAsync(This, window, name, option, value) \
    ((This)->lpVtbl->RequestCreateForWindowAsync(This, window, name, option, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerCreateWithWindowStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerStatics[] = L"Windows.Security.Credentials.IKeyCredentialManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSupportedAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        __FIAsyncOperation_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* RenewAttestationAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        HSTRING name,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption option,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** value);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_IsSupportedAsync(This, value) \
    ((This)->lpVtbl->IsSupportedAsync(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_RenewAttestationAsync(This, operation) \
    ((This)->lpVtbl->RenewAttestationAsync(This, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_RequestCreateAsync(This, name, option, value) \
    ((This)->lpVtbl->RequestCreateAsync(This, name, option, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_OpenAsync(This, name, value) \
    ((This)->lpVtbl->OpenAsync(This, name, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_DeleteAsync(This, name, operation) \
    ((This)->lpVtbl->DeleteAsync(This, name, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IKeyCredentialManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialManagerStatics2[] = L"Windows.Security.Credentials.IKeyCredentialManagerStatics2";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestCreateAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        HSTRING name,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialCreationOption option,
        HSTRING algorithm,
        HSTRING message,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialCacheConfiguration* cacheConfiguration,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CChallengeResponseKind callbackType,
        __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* attestationCallback,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** operation);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        HSTRING name,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CChallengeResponseKind callbackType,
        __x_ABI_CWindows_CSecurity_CCredentials_CIAttestationChallengeHandler* attestationCallback,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialRetrievalResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetSecureId)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_RequestCreateAsync(This, name, option, algorithm, message, cacheConfiguration, windowId, callbackType, attestationCallback, operation) \
    ((This)->lpVtbl->RequestCreateAsync(This, name, option, algorithm, message, cacheConfiguration, windowId, callbackType, attestationCallback, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_OpenAsync(This, name, callbackType, attestationCallback, operation) \
    ((This)->lpVtbl->OpenAsync(This, name, callbackType, attestationCallback, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_GetSecureId(This, result) \
    ((This)->lpVtbl->GetSecureId(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialOperationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialOperationResult[] = L"Windows.Security.Credentials.IKeyCredentialOperationResult";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialRetrievalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredentialRetrievalResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialRetrievalResult[] = L"Windows.Security.Credentials.IKeyCredentialRetrievalResult";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Credential)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredential** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CKeyCredentialStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResultVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_get_Credential(This, value) \
    ((This)->lpVtbl->get_Credential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialRetrievalResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IKeyCredentialWithWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.KeyCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IKeyCredentialWithWindow[] = L"Windows.Security.Credentials.IKeyCredentialWithWindow";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestSignForWindowAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow* This,
        struct __x_ABI_CWindows_CUI_CWindowId window,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* data,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CKeyCredentialOperationResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindowVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_RequestSignForWindowAsync(This, window, data, value) \
    ((This)->lpVtbl->RequestSignForWindowAsync(This, window, data, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIKeyCredentialWithWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Security.Credentials.IPasswordCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IPasswordCredential[] = L"Windows.Security.Credentials.IPasswordCredential";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Resource)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING* resource);
    HRESULT (STDMETHODCALLTYPE* put_Resource)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING resource);
    HRESULT (STDMETHODCALLTYPE* get_UserName)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING* userName);
    HRESULT (STDMETHODCALLTYPE* put_UserName)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING userName);
    HRESULT (STDMETHODCALLTYPE* get_Password)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING* password);
    HRESULT (STDMETHODCALLTYPE* put_Password)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        HSTRING password);
    HRESULT (STDMETHODCALLTYPE* RetrievePassword)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** props);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredentialVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_get_Resource(This, resource) \
    ((This)->lpVtbl->get_Resource(This, resource))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_put_Resource(This, resource) \
    ((This)->lpVtbl->put_Resource(This, resource))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_get_UserName(This, userName) \
    ((This)->lpVtbl->get_UserName(This, userName))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_put_UserName(This, userName) \
    ((This)->lpVtbl->put_UserName(This, userName))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_get_Password(This, password) \
    ((This)->lpVtbl->get_Password(This, password))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_put_Password(This, password) \
    ((This)->lpVtbl->put_Password(This, password))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_RetrievePassword(This) \
    ((This)->lpVtbl->RetrievePassword(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_get_Properties(This, props) \
    ((This)->lpVtbl->get_Properties(This, props))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IPasswordVault
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.PasswordVault
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IPasswordVault[] = L"Windows.Security.Credentials.IPasswordVault";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVaultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Add)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* credential);
    HRESULT (STDMETHODCALLTYPE* Remove)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* credential);
    HRESULT (STDMETHODCALLTYPE* Retrieve)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        HSTRING resource,
        HSTRING userName,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** credential);
    HRESULT (STDMETHODCALLTYPE* FindAllByResource)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        HSTRING resource,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials);
    HRESULT (STDMETHODCALLTYPE* FindAllByUserName)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        HSTRING userName,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials);
    HRESULT (STDMETHODCALLTYPE* RetrieveAll)(__x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault* This,
        __FIVectorView_1_Windows__CSecurity__CCredentials__CPasswordCredential** credentials);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVaultVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVaultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_Add(This, credential) \
    ((This)->lpVtbl->Add(This, credential))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_Remove(This, credential) \
    ((This)->lpVtbl->Remove(This, credential))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_Retrieve(This, resource, userName, credential) \
    ((This)->lpVtbl->Retrieve(This, resource, userName, credential))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FindAllByResource(This, resource, credentials) \
    ((This)->lpVtbl->FindAllByResource(This, resource, credentials))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_FindAllByUserName(This, userName, credentials) \
    ((This)->lpVtbl->FindAllByUserName(This, userName, credentials))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_RetrieveAll(This, credentials) \
    ((This)->lpVtbl->RetrieveAll(This, credentials))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordVault_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccount[] = L"Windows.Security.Credentials.IWebAccount";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountProvider)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_UserName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountState* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_get_WebAccountProvider(This, value) \
    ((This)->lpVtbl->get_WebAccountProvider(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_get_UserName(This, value) \
    ((This)->lpVtbl->get_UserName(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccount2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccount
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccount2[] = L"Windows.Security.Credentials.IWebAccount2";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        __FIMapView_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetPictureAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountPictureSize desizedSize,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SignOutAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* SignOutWithClientIdAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2* This,
        HSTRING clientId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_GetPictureAsync(This, desizedSize, asyncInfo) \
    ((This)->lpVtbl->GetPictureAsync(This, desizedSize, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_SignOutAsync(This, asyncInfo) \
    ((This)->lpVtbl->SignOutAsync(This, asyncInfo))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_SignOutWithClientIdAsync(This, clientId, asyncInfo) \
    ((This)->lpVtbl->SignOutWithClientIdAsync(This, clientId, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountFactory[] = L"Windows.Security.Credentials.IWebAccountFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWebAccount)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* webAccountProvider,
        HSTRING userName,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CWebAccountState state,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** instance);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_CreateWebAccount(This, webAccountProvider, userName, state, instance) \
    ((This)->lpVtbl->CreateWebAccount(This, webAccountProvider, userName, state, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider[] = L"Windows.Security.Credentials.IWebAccountProvider";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IconUri may be altered or unavailable for releases after Windows 8.2. Instead, use Icon.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_IconUri)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IconUri may be altered or unavailable for releases after Windows 8.2. Instead, use Icon.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_get_IconUri(This, value) \
    ((This)->lpVtbl->get_IconUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider2[] = L"Windows.Security.Credentials.IWebAccountProvider2";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayPurpose)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Authority)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_get_DisplayPurpose(This, value) \
    ((This)->lpVtbl->get_DisplayPurpose(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_get_Authority(This, value) \
    ((This)->lpVtbl->get_Authority(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Credentials.IWebAccountProvider2
 *     Windows.Security.Credentials.IWebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider3[] = L"Windows.Security.Credentials.IWebAccountProvider3";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3* This,
        __x_ABI_CWindows_CSystem_CIUser** user);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_get_User(This, user) \
    ((This)->lpVtbl->get_User(This, user))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProvider4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProvider4[] = L"Windows.Security.Credentials.IWebAccountProvider4";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSystemProvider)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4Vtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_get_IsSystemProvider(This, value) \
    ((This)->lpVtbl->get_IsSystemProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Security.Credentials.IWebAccountProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.WebAccountProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_IWebAccountProviderFactory[] = L"Windows.Security.Credentials.IWebAccountProviderFactory";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWebAccountProvider)(__x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory* This,
        HSTRING id,
        HSTRING displayName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* iconUri,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** instance);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_CreateWebAccountProvider(This, id, displayName, iconUri, instance) \
    ((This)->lpVtbl->CreateWebAccountProvider(This, id, displayName, iconUri, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredential ** Default Interface **
 *    Windows.Security.Credentials.IKeyCredential2
 *    Windows.Security.Credentials.IKeyCredentialWithWindow
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredential_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredential[] = L"Windows.Security.Credentials.KeyCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialAttestationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialAttestationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialAttestationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialAttestationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialAttestationResult[] = L"Windows.Security.Credentials.KeyCredentialAttestationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialCacheConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IKeyCredentialCacheConfigurationFactory interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialCacheConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialCacheConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialCacheConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialCacheConfiguration[] = L"Windows.Security.Credentials.KeyCredentialCacheConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerCreateWithWindowStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Credentials.IKeyCredentialManagerStatics2 interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialManager[] = L"Windows.Security.Credentials.KeyCredentialManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialOperationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialOperationResult[] = L"Windows.Security.Credentials.KeyCredentialOperationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.KeyCredentialRetrievalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IKeyCredentialRetrievalResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialRetrievalResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_KeyCredentialRetrievalResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_KeyCredentialRetrievalResult[] = L"Windows.Security.Credentials.KeyCredentialRetrievalResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.ICredentialFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IPasswordCredential ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordCredential_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordCredential[] = L"Windows.Security.Credentials.PasswordCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordCredentialPropertyStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IPropertySet ** Default Interface **
 *    Windows.Foundation.Collections.IObservableMap`2<String, Object>
 *    Windows.Foundation.Collections.IMap`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordCredentialPropertyStore_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordCredentialPropertyStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordCredentialPropertyStore[] = L"Windows.Security.Credentials.PasswordCredentialPropertyStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.PasswordVault
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IPasswordVault ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_PasswordVault_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_PasswordVault_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_PasswordVault[] = L"Windows.Security.Credentials.PasswordVault";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.WebAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IWebAccountFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IWebAccount ** Default Interface **
 *    Windows.Security.Credentials.IWebAccount2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_WebAccount_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_WebAccount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_WebAccount[] = L"Windows.Security.Credentials.WebAccount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.WebAccountProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Credentials.IWebAccountProviderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.IWebAccountProvider ** Default Interface **
 *    Windows.Security.Credentials.IWebAccountProvider2
 *    Windows.Security.Credentials.IWebAccountProvider3
 *    Windows.Security.Credentials.IWebAccountProvider4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_WebAccountProvider_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_WebAccountProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_WebAccountProvider[] = L"Windows.Security.Credentials.WebAccountProvider";
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
#endif // __windows2Esecurity2Ecredentials_p_h__

#endif // __windows2Esecurity2Ecredentials_h__
