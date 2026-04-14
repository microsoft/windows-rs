
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
#ifndef __windows2Esecurity2Eauthentication2Eidentity2Eprovider_h__
#define __windows2Esecurity2Eauthentication2Eidentity2Eprovider_h__
#ifndef __windows2Esecurity2Eauthentication2Eidentity2Eprovider_p_h__
#define __windows2Esecurity2Eauthentication2Eidentity2Eprovider_p_h__


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
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorAuthentication;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthentication

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorAuthenticationResult;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorAuthenticationStageInfo;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageInfo

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorAuthenticationStatics;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorInfo;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorInfo

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorInfo2;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2 ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorInfo2

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorRegistration;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistration

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorRegistrationResult;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistrationResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        interface ISecondaryAuthenticationFactorRegistrationStatics;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistrationStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorInfo;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0eedbda6-2de0-50af-abc4-46073245fb2d"))
IIterator<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43b7bbe4-f096-53dd-8c16-1faa4b468c86"))
IIterable<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63c2e57e-3b00-5752-8fa7-cb9cbe8fe088"))
IVectorView<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorInfo*> __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("47eb155b-abe0-55a5-9310-feb1dd57dca5"))
IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*> __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("06752d25-d43e-5d2e-a305-4e1576846fee"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorAuthenticationResult;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("18b0a73c-db59-5279-a76d-02416b2d90b6"))
IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2547373d-9684-5e5b-a9b8-a6f90ce632ad"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorAuthenticationStageInfo;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("864a2317-b440-5e9e-ae55-4550bb6307df"))
IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7811d384-2eb8-58f1-afed-4b4b888f4357"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageInfo*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus : int SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7f37ecea-e3e8-53fc-b0e5-7aa471970edd"))
IAsyncOperation<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> : IAsyncOperation_impl<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2294a212-9061-5e99-a226-a44ac8f8f4dd"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorFinishAuthenticationStatus : int SecondaryAuthenticationFactorFinishAuthenticationStatus;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9159437a-4397-546e-be61-2ef161717e06"))
IAsyncOperation<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus> : IAsyncOperation_impl<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ae1d7146-3d91-50e3-8f13-613cf2801207"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorFinishAuthenticationStatus> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorRegistrationResult;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("05da520c-aba4-584c-bc08-19c5389a70e2"))
IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistrationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a04902e8-f830-50ea-89ea-96e2a6fb9453"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistrationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10409b3c-42e4-586f-84c1-803da23765af"))
IEventHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs*, ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs*> __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_t;
#define __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_USE */

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
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorAuthenticationMessage : int SecondaryAuthenticationFactorAuthenticationMessage;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorAuthenticationScenario : int SecondaryAuthenticationFactorAuthenticationScenario;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorAuthenticationStage : int SecondaryAuthenticationFactorAuthenticationStage;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorAuthenticationStatus : int SecondaryAuthenticationFactorAuthenticationStatus;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorDeviceCapabilities : unsigned int SecondaryAuthenticationFactorDeviceCapabilities;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorDeviceFindScope : int SecondaryAuthenticationFactorDeviceFindScope;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorDevicePresence : int SecondaryAuthenticationFactorDevicePresence;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorDevicePresenceMonitoringMode : int SecondaryAuthenticationFactorDevicePresenceMonitoringMode;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        typedef enum SecondaryAuthenticationFactorRegistrationStatus : int SecondaryAuthenticationFactorRegistrationStatus;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorAuthentication;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        class SecondaryAuthenticationFactorRegistration;
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationMessage is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorAuthenticationMessage : int
                        {
                            SecondaryAuthenticationFactorAuthenticationMessage_Invalid = 0,
                            SecondaryAuthenticationFactorAuthenticationMessage_SwipeUpWelcome = 1,
                            SecondaryAuthenticationFactorAuthenticationMessage_TapWelcome = 2,
                            SecondaryAuthenticationFactorAuthenticationMessage_DeviceNeedsAttention = 3,
                            SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevice = 4,
                            SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevicePluggedin = 5,
                            SecondaryAuthenticationFactorAuthenticationMessage_BluetoothIsDisabled = 6,
                            SecondaryAuthenticationFactorAuthenticationMessage_NfcIsDisabled = 7,
                            SecondaryAuthenticationFactorAuthenticationMessage_WiFiIsDisabled = 8,
                            SecondaryAuthenticationFactorAuthenticationMessage_ExtraTapIsRequired = 9,
                            SecondaryAuthenticationFactorAuthenticationMessage_DisabledByPolicy = 10,
                            SecondaryAuthenticationFactorAuthenticationMessage_TapOnDeviceRequired = 11,
                            SecondaryAuthenticationFactorAuthenticationMessage_HoldFinger = 12,
                            SecondaryAuthenticationFactorAuthenticationMessage_ScanFinger = 13,
                            SecondaryAuthenticationFactorAuthenticationMessage_UnauthorizedUser = 14,
                            SecondaryAuthenticationFactorAuthenticationMessage_ReregisterRequired = 15,
                            SecondaryAuthenticationFactorAuthenticationMessage_TryAgain = 16,
                            SecondaryAuthenticationFactorAuthenticationMessage_SayPassphrase = 17,
                            SecondaryAuthenticationFactorAuthenticationMessage_ReadyToSignIn = 18,
                            SecondaryAuthenticationFactorAuthenticationMessage_UseAnotherSignInOption = 19,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_ConnectionRequired = 20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_TimeLimitExceeded = 21,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_CanceledByUser = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_CenterHand = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_MoveHandCloser = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_MoveHandFarther = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_PlaceHandAbove = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_RecognitionFailed = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                            SecondaryAuthenticationFactorAuthenticationMessage_DeviceUnavailable = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationScenario is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorAuthenticationScenario : int
                        {
                            SecondaryAuthenticationFactorAuthenticationScenario_SignIn = 0,
                            SecondaryAuthenticationFactorAuthenticationScenario_CredentialPrompt = 1,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationStage is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorAuthenticationStage : int
                        {
                            SecondaryAuthenticationFactorAuthenticationStage_NotStarted = 0,
                            SecondaryAuthenticationFactorAuthenticationStage_WaitingForUserConfirmation = 1,
                            SecondaryAuthenticationFactorAuthenticationStage_CollectingCredential = 2,
                            SecondaryAuthenticationFactorAuthenticationStage_SuspendingAuthentication = 3,
                            SecondaryAuthenticationFactorAuthenticationStage_CredentialCollected = 4,
                            SecondaryAuthenticationFactorAuthenticationStage_CredentialAuthenticated = 5,
                            SecondaryAuthenticationFactorAuthenticationStage_StoppingAuthentication = 6,
                            SecondaryAuthenticationFactorAuthenticationStage_ReadyForLock = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                            SecondaryAuthenticationFactorAuthenticationStage_CheckingDevicePresence
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATEDENUMERATOR("CheckingDevicePresence is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorAuthenticationStatus : int
                        {
                            SecondaryAuthenticationFactorAuthenticationStatus_Failed = 0,
                            SecondaryAuthenticationFactorAuthenticationStatus_Started = 1,
                            SecondaryAuthenticationFactorAuthenticationStatus_UnknownDevice = 2,
                            SecondaryAuthenticationFactorAuthenticationStatus_DisabledByPolicy = 3,
                            SecondaryAuthenticationFactorAuthenticationStatus_InvalidAuthenticationStage = 4,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorDeviceCapabilities is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorDeviceCapabilities : unsigned int
                        {
                            SecondaryAuthenticationFactorDeviceCapabilities_None = 0,
                            SecondaryAuthenticationFactorDeviceCapabilities_SecureStorage = 0x1,
                            SecondaryAuthenticationFactorDeviceCapabilities_StoreKeys = 0x2,
                            SecondaryAuthenticationFactorDeviceCapabilities_ConfirmUserIntentToAuthenticate = 0x4,
                            SecondaryAuthenticationFactorDeviceCapabilities_SupportSecureUserPresenceCheck
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATEDENUMERATOR("SupportSecureUserPresenceCheck is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            = 0x8,
                            SecondaryAuthenticationFactorDeviceCapabilities_TransmittedDataIsEncrypted = 0x10,
                            SecondaryAuthenticationFactorDeviceCapabilities_HMacSha256 = 0x20,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                            SecondaryAuthenticationFactorDeviceCapabilities_CloseRangeDataTransmission = 0x40,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        };

                        DEFINE_ENUM_FLAG_OPERATORS(SecondaryAuthenticationFactorDeviceCapabilities)
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceFindScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorDeviceFindScope is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorDeviceFindScope : int
                        {
                            SecondaryAuthenticationFactorDeviceFindScope_User = 0,
                            SecondaryAuthenticationFactorDeviceFindScope_AllUsers = 1,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        DEPRECATED("SecondaryAuthenticationFactorDevicePresence is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        SecondaryAuthenticationFactorDevicePresence : int
                        {
                            SecondaryAuthenticationFactorDevicePresence_Absent = 0,
                            SecondaryAuthenticationFactorDevicePresence_Present = 1,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        SecondaryAuthenticationFactorDevicePresenceMonitoringMode : int
                        {
                            SecondaryAuthenticationFactorDevicePresenceMonitoringMode_Unsupported = 0,
                            SecondaryAuthenticationFactorDevicePresenceMonitoringMode_AppManaged = 1,
                            SecondaryAuthenticationFactorDevicePresenceMonitoringMode_SystemManaged = 2,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus : int
                        {
                            SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Unsupported = 0,
                            SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Succeeded = 1,
                            SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_DisabledByPolicy = 2,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorFinishAuthenticationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorFinishAuthenticationStatus : int
                        {
                            SecondaryAuthenticationFactorFinishAuthenticationStatus_Failed = 0,
                            SecondaryAuthenticationFactorFinishAuthenticationStatus_Completed = 1,
                            SecondaryAuthenticationFactorFinishAuthenticationStatus_NonceExpired = 2,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        SecondaryAuthenticationFactorRegistrationStatus : int
                        {
                            SecondaryAuthenticationFactorRegistrationStatus_Failed = 0,
                            SecondaryAuthenticationFactorRegistrationStatus_Started = 1,
                            SecondaryAuthenticationFactorRegistrationStatus_CanceledByUser = 2,
                            SecondaryAuthenticationFactorRegistrationStatus_PinSetupRequired = 3,
                            SecondaryAuthenticationFactorRegistrationStatus_DisabledByPolicy = 4,
                        };
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthentication[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("020a16e5-6a25-40a3-8c00-50a023f619d1")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorAuthentication : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_ServiceAuthenticationHmac(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_SessionNonce(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceNonce(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceConfigurationData(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE FinishAuthenticationAsync(
                                ABI::Windows::Storage::Streams::IBuffer* deviceHmac,
                                ABI::Windows::Storage::Streams::IBuffer* sessionHmac,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE AbortAuthenticationAsync(
                                HSTRING errorLogMessage,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorAuthentication = __uuidof(ISecondaryAuthenticationFactorAuthentication);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationResult[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("9cbb5987-ef6d-4bc2-bf49-4617515a0f9a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorAuthenticationResult : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStatus* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Authentication(
                                ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthentication** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorAuthenticationResult = __uuidof(ISecondaryAuthenticationFactorAuthenticationResult);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("d4a5ee56-7291-4073-bc1f-ccb8f5afdf96")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_StageInfo(
                                ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorAuthenticationStageInfo** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs = __uuidof(ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStageInfo[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("56fec28b-e8aa-4c0f-8e4c-a559e73add88")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorAuthenticationStageInfo : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Stage(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationStage* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Scenario(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationScenario* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorAuthenticationStageInfo = __uuidof(ISecondaryAuthenticationFactorAuthenticationStageInfo);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("3f582656-28f8-4e0f-ae8c-5898b9ae2469")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorAuthenticationStatics : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE ShowNotificationMessageAsync(
                                HSTRING deviceName,
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorAuthenticationMessage message,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE StartAuthenticationAsync(
                                HSTRING deviceId,
                                ABI::Windows::Storage::Streams::IBuffer* serviceAuthenticationNonce,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult** operation
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE add_AuthenticationStageChanged(
                                __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE remove_AuthenticationStageChanged(
                                EventRegistrationToken token
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE GetAuthenticationStageInfoAsync(
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorAuthenticationStatics = __uuidof(ISecondaryAuthenticationFactorAuthenticationStatics);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("90499a19-7ef2-4523-951c-a417a24acf93")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("RegisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE RegisterDevicePresenceMonitoringAsync(
                                HSTRING deviceId,
                                HSTRING deviceInstancePath,
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringMode monitoringMode,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus** operation
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("RegisterDevicePresenceMonitoringWithNewDeviceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE RegisterDevicePresenceMonitoringWithNewDeviceAsync(
                                HSTRING deviceId,
                                HSTRING deviceInstancePath,
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringMode monitoringMode,
                                HSTRING deviceFriendlyName,
                                HSTRING deviceModelNumber,
                                ABI::Windows::Storage::Streams::IBuffer* deviceConfigurationData,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus** operation
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("UnregisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE UnregisterDevicePresenceMonitoringAsync(
                                HSTRING deviceId,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("IsDevicePresenceMonitoringSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE IsDevicePresenceMonitoringSupported(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics = __uuidof(ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorInfo[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("1e2ba861-8533-4fce-839b-ecb72410ac14")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorInfo : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                                HSTRING* deviceId
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceFriendlyName(
                                HSTRING* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceModelNumber(
                                HSTRING* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_DeviceConfigurationData(
                                ABI::Windows::Storage::Streams::IBuffer** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorInfo = __uuidof(ISecondaryAuthenticationFactorInfo);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorInfo2[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("14d981a3-fc26-4ff7-abc3-48e82a512a0a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorInfo2 : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("PresenceMonitoringMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE get_PresenceMonitoringMode(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresenceMonitoringMode* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("UpdateDevicePresenceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE UpdateDevicePresenceAsync(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDevicePresence presenceState,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            DEPRECATED("IsAuthenticationSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                            virtual HRESULT STDMETHODCALLTYPE get_IsAuthenticationSupported(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorInfo2 = __uuidof(ISecondaryAuthenticationFactorInfo2);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistration[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("9f4cbbb4-8cba-48b0-840d-dbb22a54c678")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorRegistration : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE FinishRegisteringDeviceAsync(
                                ABI::Windows::Storage::Streams::IBuffer* deviceConfigurationData,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE AbortRegisteringDeviceAsync(
                                HSTRING errorLogMessage,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorRegistration = __uuidof(ISecondaryAuthenticationFactorRegistration);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistrationResult[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("a4fe35f0-ade3-4981-af6b-ec195921682a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorRegistrationResult : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorRegistrationStatus* value
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE get_Registration(
                                ABI::Windows::Security::Authentication::Identity::Provider::ISecondaryAuthenticationFactorRegistration** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorRegistrationResult = __uuidof(ISecondaryAuthenticationFactorRegistrationResult);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistrationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    namespace Provider {
                        MIDL_INTERFACE("1adf0f65-e3b7-4155-997f-b756ef65beba")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                        ISecondaryAuthenticationFactorRegistrationStatics : public IInspectable
                        {
                        public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE RequestStartRegisteringDeviceAsync(
                                HSTRING deviceId,
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDeviceCapabilities capabilities,
                                HSTRING deviceFriendlyName,
                                HSTRING deviceModelNumber,
                                ABI::Windows::Storage::Streams::IBuffer* deviceKey,
                                ABI::Windows::Storage::Streams::IBuffer* mutualAuthenticationKey,
                                __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult** operation
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE FindAllRegisteredDeviceInfoAsync(
                                ABI::Windows::Security::Authentication::Identity::Provider::SecondaryAuthenticationFactorDeviceFindScope queryType,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo** deviceInfoList
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE UnregisterDeviceAsync(
                                HSTRING deviceId,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                            virtual HRESULT STDMETHODCALLTYPE UpdateDeviceConfigurationDataAsync(
                                HSTRING deviceId,
                                ABI::Windows::Storage::Streams::IBuffer* deviceConfigurationData,
                                ABI::Windows::Foundation::IAsyncAction** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_ISecondaryAuthenticationFactorRegistrationStatics = __uuidof(ISecondaryAuthenticationFactorRegistrationStatics);
                    } /* Provider */
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo ** Default Interface **
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2 __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorFinishAuthenticationStatus __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorFinishAuthenticationStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorFinishAuthenticationStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;

typedef struct __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationMessage __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationMessage;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationScenario __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationScenario;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStage __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStage;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStatus __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStatus;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceCapabilities __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceCapabilities;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceFindScope __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceFindScope;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresence __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresence;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorRegistrationStatus __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorRegistrationStatus;

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationMessage is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationMessage
{
    SecondaryAuthenticationFactorAuthenticationMessage_Invalid = 0,
    SecondaryAuthenticationFactorAuthenticationMessage_SwipeUpWelcome = 1,
    SecondaryAuthenticationFactorAuthenticationMessage_TapWelcome = 2,
    SecondaryAuthenticationFactorAuthenticationMessage_DeviceNeedsAttention = 3,
    SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevice = 4,
    SecondaryAuthenticationFactorAuthenticationMessage_LookingForDevicePluggedin = 5,
    SecondaryAuthenticationFactorAuthenticationMessage_BluetoothIsDisabled = 6,
    SecondaryAuthenticationFactorAuthenticationMessage_NfcIsDisabled = 7,
    SecondaryAuthenticationFactorAuthenticationMessage_WiFiIsDisabled = 8,
    SecondaryAuthenticationFactorAuthenticationMessage_ExtraTapIsRequired = 9,
    SecondaryAuthenticationFactorAuthenticationMessage_DisabledByPolicy = 10,
    SecondaryAuthenticationFactorAuthenticationMessage_TapOnDeviceRequired = 11,
    SecondaryAuthenticationFactorAuthenticationMessage_HoldFinger = 12,
    SecondaryAuthenticationFactorAuthenticationMessage_ScanFinger = 13,
    SecondaryAuthenticationFactorAuthenticationMessage_UnauthorizedUser = 14,
    SecondaryAuthenticationFactorAuthenticationMessage_ReregisterRequired = 15,
    SecondaryAuthenticationFactorAuthenticationMessage_TryAgain = 16,
    SecondaryAuthenticationFactorAuthenticationMessage_SayPassphrase = 17,
    SecondaryAuthenticationFactorAuthenticationMessage_ReadyToSignIn = 18,
    SecondaryAuthenticationFactorAuthenticationMessage_UseAnotherSignInOption = 19,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_ConnectionRequired = 20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_TimeLimitExceeded = 21,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_CanceledByUser = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_CenterHand = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_MoveHandCloser = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_MoveHandFarther = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_PlaceHandAbove = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_RecognitionFailed = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    SecondaryAuthenticationFactorAuthenticationMessage_DeviceUnavailable = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationScenario is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationScenario
{
    SecondaryAuthenticationFactorAuthenticationScenario_SignIn = 0,
    SecondaryAuthenticationFactorAuthenticationScenario_CredentialPrompt = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStage is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStage
{
    SecondaryAuthenticationFactorAuthenticationStage_NotStarted = 0,
    SecondaryAuthenticationFactorAuthenticationStage_WaitingForUserConfirmation = 1,
    SecondaryAuthenticationFactorAuthenticationStage_CollectingCredential = 2,
    SecondaryAuthenticationFactorAuthenticationStage_SuspendingAuthentication = 3,
    SecondaryAuthenticationFactorAuthenticationStage_CredentialCollected = 4,
    SecondaryAuthenticationFactorAuthenticationStage_CredentialAuthenticated = 5,
    SecondaryAuthenticationFactorAuthenticationStage_StoppingAuthentication = 6,
    SecondaryAuthenticationFactorAuthenticationStage_ReadyForLock = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    SecondaryAuthenticationFactorAuthenticationStage_CheckingDevicePresence
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATEDENUMERATOR("CheckingDevicePresence is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStatus
{
    SecondaryAuthenticationFactorAuthenticationStatus_Failed = 0,
    SecondaryAuthenticationFactorAuthenticationStatus_Started = 1,
    SecondaryAuthenticationFactorAuthenticationStatus_UnknownDevice = 2,
    SecondaryAuthenticationFactorAuthenticationStatus_DisabledByPolicy = 3,
    SecondaryAuthenticationFactorAuthenticationStatus_InvalidAuthenticationStage = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorDeviceCapabilities is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceCapabilities
{
    SecondaryAuthenticationFactorDeviceCapabilities_None = 0,
    SecondaryAuthenticationFactorDeviceCapabilities_SecureStorage = 0x1,
    SecondaryAuthenticationFactorDeviceCapabilities_StoreKeys = 0x2,
    SecondaryAuthenticationFactorDeviceCapabilities_ConfirmUserIntentToAuthenticate = 0x4,
    SecondaryAuthenticationFactorDeviceCapabilities_SupportSecureUserPresenceCheck
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATEDENUMERATOR("SupportSecureUserPresenceCheck is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    = 0x8,
    SecondaryAuthenticationFactorDeviceCapabilities_TransmittedDataIsEncrypted = 0x10,
    SecondaryAuthenticationFactorDeviceCapabilities_HMacSha256 = 0x20,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    SecondaryAuthenticationFactorDeviceCapabilities_CloseRangeDataTransmission = 0x40,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDeviceFindScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorDeviceFindScope is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceFindScope
{
    SecondaryAuthenticationFactorDeviceFindScope_User = 0,
    SecondaryAuthenticationFactorDeviceFindScope_AllUsers = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
DEPRECATED("SecondaryAuthenticationFactorDevicePresence is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresence
{
    SecondaryAuthenticationFactorDevicePresence_Absent = 0,
    SecondaryAuthenticationFactorDevicePresence_Present = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode
{
    SecondaryAuthenticationFactorDevicePresenceMonitoringMode_Unsupported = 0,
    SecondaryAuthenticationFactorDevicePresenceMonitoringMode_AppManaged = 1,
    SecondaryAuthenticationFactorDevicePresenceMonitoringMode_SystemManaged = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus
{
    SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Unsupported = 0,
    SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_Succeeded = 1,
    SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus_DisabledByPolicy = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorFinishAuthenticationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorFinishAuthenticationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorFinishAuthenticationStatus
{
    SecondaryAuthenticationFactorFinishAuthenticationStatus_Failed = 0,
    SecondaryAuthenticationFactorFinishAuthenticationStatus_Completed = 1,
    SecondaryAuthenticationFactorFinishAuthenticationStatus_NonceExpired = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorRegistrationStatus
{
    SecondaryAuthenticationFactorRegistrationStatus_Failed = 0,
    SecondaryAuthenticationFactorRegistrationStatus_Started = 1,
    SecondaryAuthenticationFactorRegistrationStatus_CanceledByUser = 2,
    SecondaryAuthenticationFactorRegistrationStatus_PinSetupRequired = 3,
    SecondaryAuthenticationFactorRegistrationStatus_DisabledByPolicy = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthentication[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_ServiceAuthenticationHmac)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_SessionNonce)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceNonce)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceConfigurationData)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* FinishAuthenticationAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* deviceHmac,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* sessionHmac,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorFinishAuthenticationStatus** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* AbortAuthenticationAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication* This,
        HSTRING errorLogMessage,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_get_ServiceAuthenticationHmac(This, value) \
    ((This)->lpVtbl->get_ServiceAuthenticationHmac(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_get_SessionNonce(This, value) \
    ((This)->lpVtbl->get_SessionNonce(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_get_DeviceNonce(This, value) \
    ((This)->lpVtbl->get_DeviceNonce(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_get_DeviceConfigurationData(This, value) \
    ((This)->lpVtbl->get_DeviceConfigurationData(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_FinishAuthenticationAsync(This, deviceHmac, sessionHmac, result) \
    ((This)->lpVtbl->FinishAuthenticationAsync(This, deviceHmac, sessionHmac, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_AbortAuthenticationAsync(This, errorLogMessage, result) \
    ((This)->lpVtbl->AbortAuthenticationAsync(This, errorLogMessage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationResult[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStatus* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Authentication)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthentication** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_get_Authentication(This, value) \
    ((This)->lpVtbl->get_Authentication(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_StageInfo)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_get_StageInfo(This, value) \
    ((This)->lpVtbl->get_StageInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStageInfo[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Stage)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationStage* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Scenario)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationScenario* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_get_Stage(This, value) \
    ((This)->lpVtbl->get_Stage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_get_Scenario(This, value) \
    ((This)->lpVtbl->get_Scenario(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorAuthenticationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowNotificationMessageAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        HSTRING deviceName,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorAuthenticationMessage message,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* StartAuthenticationAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* serviceAuthenticationNonce,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationResult** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* add_AuthenticationStageChanged)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        __FIEventHandler_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageChangedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* remove_AuthenticationStageChanged)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetAuthenticationStageInfoAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorAuthenticationStageInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_ShowNotificationMessageAsync(This, deviceName, message, result) \
    ((This)->lpVtbl->ShowNotificationMessageAsync(This, deviceName, message, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_StartAuthenticationAsync(This, deviceId, serviceAuthenticationNonce, operation) \
    ((This)->lpVtbl->StartAuthenticationAsync(This, deviceId, serviceAuthenticationNonce, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_add_AuthenticationStageChanged(This, handler, token) \
    ((This)->lpVtbl->add_AuthenticationStageChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_remove_AuthenticationStageChanged(This, token) \
    ((This)->lpVtbl->remove_AuthenticationStageChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_GetAuthenticationStageInfoAsync(This, result) \
    ((This)->lpVtbl->GetAuthenticationStageInfoAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorAuthenticationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorDevicePresenceMonitoringRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("RegisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* RegisterDevicePresenceMonitoringAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        HSTRING deviceId,
        HSTRING deviceInstancePath,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode monitoringMode,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("RegisterDevicePresenceMonitoringWithNewDeviceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* RegisterDevicePresenceMonitoringWithNewDeviceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        HSTRING deviceId,
        HSTRING deviceInstancePath,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode monitoringMode,
        HSTRING deviceFriendlyName,
        HSTRING deviceModelNumber,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* deviceConfigurationData,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("UnregisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* UnregisterDevicePresenceMonitoringAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("IsDevicePresenceMonitoringSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* IsDevicePresenceMonitoringSupported)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("RegisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_RegisterDevicePresenceMonitoringAsync(This, deviceId, deviceInstancePath, monitoringMode, operation) \
    ((This)->lpVtbl->RegisterDevicePresenceMonitoringAsync(This, deviceId, deviceInstancePath, monitoringMode, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("RegisterDevicePresenceMonitoringWithNewDeviceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_RegisterDevicePresenceMonitoringWithNewDeviceAsync(This, deviceId, deviceInstancePath, monitoringMode, deviceFriendlyName, deviceModelNumber, deviceConfigurationData, operation) \
    ((This)->lpVtbl->RegisterDevicePresenceMonitoringWithNewDeviceAsync(This, deviceId, deviceInstancePath, monitoringMode, deviceFriendlyName, deviceModelNumber, deviceConfigurationData, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("UnregisterDevicePresenceMonitoringAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_UnregisterDevicePresenceMonitoringAsync(This, deviceId, result) \
    ((This)->lpVtbl->UnregisterDevicePresenceMonitoringAsync(This, deviceId, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("IsDevicePresenceMonitoringSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_IsDevicePresenceMonitoringSupported(This, value) \
    ((This)->lpVtbl->IsDevicePresenceMonitoringSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorInfo[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        HSTRING* deviceId);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceFriendlyName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceModelNumber)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_DeviceConfigurationData)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_get_DeviceId(This, deviceId) \
    ((This)->lpVtbl->get_DeviceId(This, deviceId))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_get_DeviceFriendlyName(This, value) \
    ((This)->lpVtbl->get_DeviceFriendlyName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_get_DeviceModelNumber(This, value) \
    ((This)->lpVtbl->get_DeviceModelNumber(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_get_DeviceConfigurationData(This, value) \
    ((This)->lpVtbl->get_DeviceConfigurationData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorInfo2[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("PresenceMonitoringMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_PresenceMonitoringMode)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresenceMonitoringMode* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("UpdateDevicePresenceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* UpdateDevicePresenceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDevicePresence presenceState,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("IsAuthenticationSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_IsAuthenticationSupported)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("PresenceMonitoringMode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_get_PresenceMonitoringMode(This, value) \
    ((This)->lpVtbl->get_PresenceMonitoringMode(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("UpdateDevicePresenceAsync is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_UpdateDevicePresenceAsync(This, presenceState, result) \
    ((This)->lpVtbl->UpdateDevicePresenceAsync(This, presenceState, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    DEPRECATED("IsAuthenticationSupported is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_get_IsAuthenticationSupported(This, value) \
    ((This)->lpVtbl->get_IsAuthenticationSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistration[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* FinishRegisteringDeviceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* deviceConfigurationData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* AbortRegisteringDeviceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration* This,
        HSTRING errorLogMessage,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_FinishRegisteringDeviceAsync(This, deviceConfigurationData, result) \
    ((This)->lpVtbl->FinishRegisteringDeviceAsync(This, deviceConfigurationData, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_AbortRegisteringDeviceAsync(This, errorLogMessage, result) \
    ((This)->lpVtbl->AbortRegisteringDeviceAsync(This, errorLogMessage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistrationResult[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorRegistrationStatus* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Registration)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistration** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_get_Registration(This, value) \
    ((This)->lpVtbl->get_Registration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_Provider_ISecondaryAuthenticationFactorRegistrationStatics[] = L"Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* RequestStartRegisteringDeviceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceCapabilities capabilities,
        HSTRING deviceFriendlyName,
        HSTRING deviceModelNumber,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* deviceKey,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* mutualAuthenticationKey,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorRegistrationResult** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* FindAllRegisteredDeviceInfoAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CSecondaryAuthenticationFactorDeviceFindScope queryType,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CProvider__CSecondaryAuthenticationFactorInfo** deviceInfoList);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* UnregisterDeviceAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* UpdateDeviceConfigurationDataAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* deviceConfigurationData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_RequestStartRegisteringDeviceAsync(This, deviceId, capabilities, deviceFriendlyName, deviceModelNumber, deviceKey, mutualAuthenticationKey, operation) \
    ((This)->lpVtbl->RequestStartRegisteringDeviceAsync(This, deviceId, capabilities, deviceFriendlyName, deviceModelNumber, deviceKey, mutualAuthenticationKey, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_FindAllRegisteredDeviceInfoAsync(This, queryType, deviceInfoList) \
    ((This)->lpVtbl->FindAllRegisteredDeviceInfoAsync(This, queryType, deviceInfoList))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_UnregisterDeviceAsync(This, deviceId, result) \
    ((This)->lpVtbl->UnregisterDeviceAsync(This, deviceId, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_UpdateDeviceConfigurationDataAsync(This, deviceId, deviceConfigurationData, result) \
    ((This)->lpVtbl->UpdateDeviceConfigurationDataAsync(This, deviceId, deviceConfigurationData, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CProvider_CISecondaryAuthenticationFactorRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthentication is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthentication[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthentication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationResult[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorAuthenticationStageInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorAuthenticationStageInfo[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorAuthenticationStageInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo ** Default Interface **
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorInfo[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistration[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("SecondaryAuthenticationFactorRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_Provider_SecondaryAuthenticationFactorRegistrationResult[] = L"Windows.Security.Authentication.Identity.Provider.SecondaryAuthenticationFactorRegistrationResult";
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
#endif // __windows2Esecurity2Eauthentication2Eidentity2Eprovider_p_h__

#endif // __windows2Esecurity2Eauthentication2Eidentity2Eprovider_h__
