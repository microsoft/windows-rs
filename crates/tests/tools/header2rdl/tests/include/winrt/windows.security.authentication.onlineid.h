
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
#ifndef __windows2Esecurity2Eauthentication2Eonlineid_h__
#define __windows2Esecurity2Eauthentication2Eonlineid_h__
#ifndef __windows2Esecurity2Eauthentication2Eonlineid_p_h__
#define __windows2Esecurity2Eauthentication2Eonlineid_p_h__


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
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdAuthenticator;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator ABI::Windows::Security::Authentication::OnlineId::IOnlineIdAuthenticator

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdServiceTicket;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicket

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdServiceTicketRequest;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdServiceTicketRequestFactory;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequestFactory

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdSystemAuthenticatorForUser;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemAuthenticatorForUser

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdSystemAuthenticatorStatics;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemAuthenticatorStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdSystemIdentity;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemIdentity

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IOnlineIdSystemTicketResult;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemTicketResult

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    interface IUserIdentity;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity ABI::Windows::Security::Authentication::OnlineId::IUserIdentity

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class OnlineIdSystemTicketResult;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("162f5870-5a4a-503c-987f-a05a1312d8e4"))
IAsyncOperation<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemTicketResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("05f9f2ec-5950-56f8-b7f8-22e20b984679"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemTicketResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class UserIdentity;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8cc25e1-409f-57f4-bbe4-3b95b74b86c8"))
IAsyncOperation<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*, ABI::Windows::Security::Authentication::OnlineId::IUserIdentity*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.OnlineId.UserIdentity>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*> __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cded76fd-7841-52a0-a771-76cd751d13cd"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*, ABI::Windows::Security::Authentication::OnlineId::IUserIdentity*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authentication.OnlineId.UserIdentity>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Authentication::OnlineId::UserIdentity*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class OnlineIdServiceTicket;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("039feee7-0882-50e2-bc3f-4fac7e8c2769"))
IIterator<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicket*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("809d1314-97ab-5544-9891-ddcdfadd1dbb"))
IIterable<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicket*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class OnlineIdServiceTicketRequest;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6a5c8e4-6e3c-5c37-92cf-cf9f1c383335"))
IIterator<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*> __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb72d686-9516-520d-a274-fa4cd1762cb2"))
IIterable<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicketRequest*> __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("45f03233-e7a8-5ade-9ff3-0b8a1c6ba76b"))
IVectorView<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*, ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicket*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::OnlineId::OnlineIdServiceTicket*> __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
                namespace OnlineId {
                    typedef enum CredentialPromptType : int CredentialPromptType;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    typedef enum OnlineIdSystemTicketStatus : int OnlineIdSystemTicketStatus;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class OnlineIdSystemAuthenticatorForUser;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class OnlineIdSystemIdentity;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class SignOutUserOperation;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    class UserAuthenticationOperation;
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Authentication.OnlineId.CredentialPromptType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    enum CredentialPromptType : int
                    {
                        CredentialPromptType_PromptIfNeeded = 0,
                        CredentialPromptType_RetypeCredentials = 1,
                        CredentialPromptType_DoNotPrompt = 2,
                    };
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    enum OnlineIdSystemTicketStatus : int
                    {
                        OnlineIdSystemTicketStatus_Success = 0,
                        OnlineIdSystemTicketStatus_Error = 1,
                        OnlineIdSystemTicketStatus_ServiceConnectionError = 2,
                    };
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdAuthenticator[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("a003f58a-29ab-4817-b884-d7516dad18b9")
                    IOnlineIdAuthenticator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateUserAsync(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest* request,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity** authenticationOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AuthenticateUserAsyncAdvanced(
                            __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* requests,
                            ABI::Windows::Security::Authentication::OnlineId::CredentialPromptType credentialPromptType,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity** authenticationOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SignOutUserAsync(
                            ABI::Windows::Foundation::IAsyncAction** signOutUserOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ApplicationId(
                            GUID value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ApplicationId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanSignOut(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthenticatedSafeCustomerId(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdAuthenticator = __uuidof(IOnlineIdAuthenticator);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicket[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("c95c547f-d781-4a94-acb8-c59874238c26")
                    IOnlineIdServiceTicket : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdServiceTicket = __uuidof(IOnlineIdServiceTicket);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicketRequest[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("297445d3-fb63-4135-8909-4e354c061466")
                    IOnlineIdServiceTicketRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Service(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Policy(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdServiceTicketRequest = __uuidof(IOnlineIdServiceTicketRequest);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicketRequestFactory[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("bebb0a08-9e73-4077-9614-08614c0bc245")
                    IOnlineIdServiceTicketRequestFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateOnlineIdServiceTicketRequest(
                            HSTRING service,
                            HSTRING policy,
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest** onlineIdServiceTicketRequest
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateOnlineIdServiceTicketRequestAdvanced(
                            HSTRING service,
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest** onlineIdServiceTicketRequest
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdServiceTicketRequestFactory = __uuidof(IOnlineIdServiceTicketRequestFactory);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemAuthenticatorForUser[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("5798befb-1de4-4186-a2e6-b563f86aaf44")
                    IOnlineIdSystemAuthenticatorForUser : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetTicketAsync(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicketRequest* request,
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ApplicationId(
                            GUID value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ApplicationId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_User(
                            ABI::Windows::System::IUser** user
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdSystemAuthenticatorForUser = __uuidof(IOnlineIdSystemAuthenticatorForUser);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemAuthenticatorStatics[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("85047792-f634-41e3-96a4-5164e902c740")
                    IOnlineIdSystemAuthenticatorStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Default(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemAuthenticatorForUser** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetForUser(
                            ABI::Windows::System::IUser* user,
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemAuthenticatorForUser** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdSystemAuthenticatorStatics = __uuidof(IOnlineIdSystemAuthenticatorStatics);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemIdentity[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("743cd20d-b6ca-434d-8124-53ea12685307")
                    IOnlineIdSystemIdentity : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Ticket(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdServiceTicket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdSystemIdentity = __uuidof(IOnlineIdSystemIdentity);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemTicketResult[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("db0a5ff8-b098-4acd-9d13-9e640652b5b6")
                    IOnlineIdSystemTicketResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Identity(
                            ABI::Windows::Security::Authentication::OnlineId::IOnlineIdSystemIdentity** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Security::Authentication::OnlineId::OnlineIdSystemTicketStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IOnlineIdSystemTicketResult = __uuidof(IOnlineIdSystemTicketResult);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IUserIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.UserIdentity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IUserIdentity[] = L"Windows.Security.Authentication.OnlineId.IUserIdentity";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace OnlineId {
                    MIDL_INTERFACE("2146d9cd-0742-4be3-8a1c-7c7ae679aa88")
                    IUserIdentity : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Tickets(
                            __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SafeCustomerId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SignInName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FirstName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LastName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsBetaAccount(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsConfirmedPC(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserIdentity = __uuidof(IUserIdentity);
                } /* OnlineId */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator[] = L"Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket[] = L"Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest[] = L"Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.SignOutUserOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_SignOutUserOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_SignOutUserOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_SignOutUserOperation[] = L"Windows.Security.Authentication.OnlineId.SignOutUserOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.UserAuthenticationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.OnlineId.UserIdentity> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation[] = L"Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.UserIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IUserIdentity ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserIdentity_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserIdentity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_UserIdentity[] = L"Windows.Security.Authentication.OnlineId.UserIdentity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CCredentialPromptType __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CCredentialPromptType;

typedef enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_COnlineIdSystemTicketStatus __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_COnlineIdSystemTicketStatus;

/*
 *
 * Struct Windows.Security.Authentication.OnlineId.CredentialPromptType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CCredentialPromptType
{
    CredentialPromptType_PromptIfNeeded = 0,
    CredentialPromptType_RetypeCredentials = 1,
    CredentialPromptType_DoNotPrompt = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_COnlineIdSystemTicketStatus
{
    OnlineIdSystemTicketStatus_Success = 0,
    OnlineIdSystemTicketStatus_Error = 1,
    OnlineIdSystemTicketStatus_ServiceConnectionError = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdAuthenticator[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AuthenticateUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity** authenticationOperation);
    HRESULT (STDMETHODCALLTYPE* AuthenticateUserAsyncAdvanced)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        __FIIterable_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicketRequest* requests,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CCredentialPromptType credentialPromptType,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__CUserIdentity** authenticationOperation);
    HRESULT (STDMETHODCALLTYPE* SignOutUserAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** signOutUserOperation);
    HRESULT (STDMETHODCALLTYPE* put_ApplicationId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_CanSignOut)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticatedSafeCustomerId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticatorVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_AuthenticateUserAsync(This, request, authenticationOperation) \
    ((This)->lpVtbl->AuthenticateUserAsync(This, request, authenticationOperation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_AuthenticateUserAsyncAdvanced(This, requests, credentialPromptType, authenticationOperation) \
    ((This)->lpVtbl->AuthenticateUserAsyncAdvanced(This, requests, credentialPromptType, authenticationOperation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_SignOutUserAsync(This, signOutUserOperation) \
    ((This)->lpVtbl->SignOutUserAsync(This, signOutUserOperation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_put_ApplicationId(This, value) \
    ((This)->lpVtbl->put_ApplicationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_get_ApplicationId(This, value) \
    ((This)->lpVtbl->get_ApplicationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_get_CanSignOut(This, value) \
    ((This)->lpVtbl->get_CanSignOut(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_get_AuthenticatedSafeCustomerId(This, value) \
    ((This)->lpVtbl->get_AuthenticatedSafeCustomerId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdAuthenticator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicket[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest** value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicketRequest[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Service)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Policy)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_get_Service(This, value) \
    ((This)->lpVtbl->get_Service(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_get_Policy(This, value) \
    ((This)->lpVtbl->get_Policy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdServiceTicketRequestFactory[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateOnlineIdServiceTicketRequest)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        HSTRING service,
        HSTRING policy,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest** onlineIdServiceTicketRequest);
    HRESULT (STDMETHODCALLTYPE* CreateOnlineIdServiceTicketRequestAdvanced)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory* This,
        HSTRING service,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest** onlineIdServiceTicketRequest);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_CreateOnlineIdServiceTicketRequest(This, service, policy, onlineIdServiceTicketRequest) \
    ((This)->lpVtbl->CreateOnlineIdServiceTicketRequest(This, service, policy, onlineIdServiceTicketRequest))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_CreateOnlineIdServiceTicketRequestAdvanced(This, service, onlineIdServiceTicketRequest) \
    ((This)->lpVtbl->CreateOnlineIdServiceTicketRequestAdvanced(This, service, onlineIdServiceTicketRequest))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemAuthenticatorForUser[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTicketAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicketRequest* request,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdSystemTicketResult** operation);
    HRESULT (STDMETHODCALLTYPE* put_ApplicationId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** user);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUserVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_GetTicketAsync(This, request, operation) \
    ((This)->lpVtbl->GetTicketAsync(This, request, operation))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_put_ApplicationId(This, value) \
    ((This)->lpVtbl->put_ApplicationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_get_ApplicationId(This, value) \
    ((This)->lpVtbl->get_ApplicationId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_get_User(This, user) \
    ((This)->lpVtbl->get_User(This, user))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemAuthenticatorStatics[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Default)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser** value);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorForUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_get_Default(This, value) \
    ((This)->lpVtbl->get_Default(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_GetForUser(This, user, value) \
    ((This)->lpVtbl->GetForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemAuthenticatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemIdentity[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Ticket)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdServiceTicket** value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentityVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_get_Ticket(This, value) \
    ((This)->lpVtbl->get_Ticket(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IOnlineIdSystemTicketResult[] = L"Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Identity)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemIdentity** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        enum __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_COnlineIdSystemTicketStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResultVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_get_Identity(This, value) \
    ((This)->lpVtbl->get_Identity(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIOnlineIdSystemTicketResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Authentication.OnlineId.IUserIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.OnlineId.UserIdentity
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_OnlineId_IUserIdentity[] = L"Windows.Security.Authentication.OnlineId.IUserIdentity";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tickets)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__COnlineId__COnlineIdServiceTicket** value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SafeCustomerId)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SignInName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FirstName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LastName)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBetaAccount)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConfirmedPC)(__x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentityVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_Tickets(This, value) \
    ((This)->lpVtbl->get_Tickets(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_SafeCustomerId(This, value) \
    ((This)->lpVtbl->get_SafeCustomerId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_SignInName(This, value) \
    ((This)->lpVtbl->get_SignInName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_FirstName(This, value) \
    ((This)->lpVtbl->get_FirstName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_LastName(This, value) \
    ((This)->lpVtbl->get_LastName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_IsBetaAccount(This, value) \
    ((This)->lpVtbl->get_IsBetaAccount(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_get_IsConfirmedPC(This, value) \
    ((This)->lpVtbl->get_IsConfirmedPC(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_COnlineId_CIUserIdentity_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdAuthenticator[] = L"Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicket[] = L"Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdServiceTicketRequest[] = L"Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticator[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemAuthenticatorForUser[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemIdentity[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_OnlineIdSystemTicketResult[] = L"Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.SignOutUserOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_SignOutUserOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_SignOutUserOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_SignOutUserOperation[] = L"Windows.Security.Authentication.OnlineId.SignOutUserOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.UserAuthenticationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Security.Authentication.OnlineId.UserIdentity> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_UserAuthenticationOperation[] = L"Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Authentication.OnlineId.UserIdentity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.OnlineId.IUserIdentity ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserIdentity_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_OnlineId_UserIdentity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_OnlineId_UserIdentity[] = L"Windows.Security.Authentication.OnlineId.UserIdentity";
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
#endif // __windows2Esecurity2Eauthentication2Eonlineid_p_h__

#endif // __windows2Esecurity2Eauthentication2Eonlineid_h__
