
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
#ifndef __windows2Esecurity2Eauthorization2Eappcapabilityaccess_h__
#define __windows2Esecurity2Eauthorization2Eappcapabilityaccess_h__
#ifndef __windows2Esecurity2Eauthorization2Eappcapabilityaccess_p_h__
#define __windows2Esecurity2Eauthorization2Eappcapabilityaccess_p_h__


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
#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    interface IAppCapability;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapability

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    interface IAppCapability2;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2 ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapability2

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    interface IAppCapabilityAccessChangedEventArgs;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapabilityAccessChangedEventArgs

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    interface IAppCapabilityStatics;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapabilityStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    typedef enum AppCapabilityAccessStatus : int AppCapabilityAccessStatus;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c0538d02-01f7-51a1-99bd-3d148d055fa1"))
IKeyValuePair<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IKeyValuePair_impl<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e97286e-7954-5b79-bea1-83af142e4fb2"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("62e88ad9-d63e-5173-baa2-bb4521c7e82a"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("20366438-9fab-5c12-87eb-da867e383fe7"))
IMapView<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IMapView_impl<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a66001f3-e332-531a-bf49-4edd3af88de7"))
IAsyncOperation<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> : IAsyncOperation_impl<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bdf03ead-a75b-510c-87d2-5b5753bdf1bd"))
IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> : IAsyncOperationCompletedHandler_impl<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMapView`2<String, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus*> __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("827caf42-5fe6-5b5b-84ce-c44834134d3d"))
IAsyncOperation<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ea0f2e9-bc97-58e8-a3a6-c829b9e5f2aa"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000


#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    class AppCapability;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    class AppCapabilityAccessChangedEventArgs;
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6d923c95-7b83-5f59-8883-f44175284898"))
ITypedEventHandler<ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapability*, ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapability*, ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapability*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessChangedEventArgs*, ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapabilityAccessChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Security.Authorization.AppCapabilityAccess.AppCapability, Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapability*, ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessChangedEventArgs*> __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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

/*
 *
 * Struct Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    enum AppCapabilityAccessStatus : int
                    {
                        AppCapabilityAccessStatus_DeniedBySystem = 0,
                        AppCapabilityAccessStatus_NotDeclaredByApp = 1,
                        AppCapabilityAccessStatus_DeniedByUser = 2,
                        AppCapabilityAccessStatus_UserPromptRequired = 3,
                        AppCapabilityAccessStatus_Allowed = 4,
                    };
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapability[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapability";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    MIDL_INTERFACE("4c49d915-8a2a-4295-9437-2df7c396aff4")
                    IAppCapability : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CapabilityName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_User(
                            ABI::Windows::System::IUser** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                            __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CheckAccess(
                            ABI::Windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AccessChanged(
                            __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AccessChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppCapability = __uuidof(IAppCapability);
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapability2[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    MIDL_INTERFACE("11c7ccb6-c74f-50a3-b960-88008767d939")
                    IAppCapability2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayMessage(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DisplayMessage(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppCapability2 = __uuidof(IAppCapability2);
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapabilityAccessChangedEventArgs[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    MIDL_INTERFACE("0a578d15-bdd7-457e-8cca-6f53bd2e5944")
                    IAppCapabilityAccessChangedEventArgs : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAppCapabilityAccessChangedEventArgs = __uuidof(IAppCapabilityAccessChangedEventArgs);
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapabilityStatics[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authorization {
                namespace AppCapabilityAccess {
                    MIDL_INTERFACE("7c353e2a-46ee-44e5-af3d-6ad3fc49bd22")
                    IAppCapabilityStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessForCapabilitiesAsync(
                            __FIIterable_1_HSTRING* capabilityNames,
                            __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessForCapabilitiesForUserAsync(
                            ABI::Windows::System::IUser* user,
                            __FIIterable_1_HSTRING* capabilityNames,
                            __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            HSTRING capabilityName,
                            ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapability** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWithProcessIdForUser(
                            ABI::Windows::System::IUser* user,
                            HSTRING capabilityName,
                            UINT32 pid,
                            ABI::Windows::Security::Authorization::AppCapabilityAccess::IAppCapability** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppCapabilityStatics = __uuidof(IAppCapabilityStatics);
                } /* AppCapabilityAccess */
            } /* Authorization */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapability ** Default Interface **
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapability_DEFINED
#define RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authorization_AppCapabilityAccess_AppCapability[] = L"Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs[] = L"Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability;

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2 __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2;

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs;

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING key,
        enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** first,
        __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* This,
        __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* sender,
        __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

/*
 *
 * Struct Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus
{
    AppCapabilityAccessStatus_DeniedBySystem = 0,
    AppCapabilityAccessStatus_NotDeclaredByApp = 1,
    AppCapabilityAccessStatus_DeniedByUser = 2,
    AppCapabilityAccessStatus_UserPromptRequired = 3,
    AppCapabilityAccessStatus_Allowed = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapability[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapability";
typedef struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CapabilityName)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        __FIAsyncOperation_1_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* CheckAccess)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        enum __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CAppCapabilityAccessStatus* result);
    HRESULT (STDMETHODCALLTYPE* add_AccessChanged)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        __FITypedEventHandler_2_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapability_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AccessChanged)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_get_CapabilityName(This, value) \
    ((This)->lpVtbl->get_CapabilityName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_CheckAccess(This, result) \
    ((This)->lpVtbl->CheckAccess(This, result))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_add_AccessChanged(This, handler, token) \
    ((This)->lpVtbl->add_AccessChanged(This, handler, token))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_remove_AccessChanged(This, token) \
    ((This)->lpVtbl->remove_AccessChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapability2[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2";
typedef struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayMessage)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayMessage)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2Vtbl;

interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_get_DisplayMessage(This, value) \
    ((This)->lpVtbl->get_DisplayMessage(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_put_DisplayMessage(This, value) \
    ((This)->lpVtbl->put_DisplayMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapabilityAccessChangedEventArgs[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs";
typedef struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityAccessChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authorization_AppCapabilityAccess_IAppCapabilityStatics[] = L"Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessForCapabilitiesAsync)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        __FIIterable_1_HSTRING* capabilityNames,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* RequestAccessForCapabilitiesForUserAsync)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIIterable_1_HSTRING* capabilityNames,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_Windows__CSecurity__CAuthorization__CAppCapabilityAccess__CAppCapabilityAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        HSTRING capabilityName,
        __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithProcessIdForUser)(__x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING capabilityName,
        UINT32 pid,
        __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapability** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_RequestAccessForCapabilitiesAsync(This, capabilityNames, operation) \
    ((This)->lpVtbl->RequestAccessForCapabilitiesAsync(This, capabilityNames, operation))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_RequestAccessForCapabilitiesForUserAsync(This, user, capabilityNames, operation) \
    ((This)->lpVtbl->RequestAccessForCapabilitiesForUserAsync(This, user, capabilityNames, operation))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_Create(This, capabilityName, result) \
    ((This)->lpVtbl->Create(This, capabilityName, result))

#define __x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_CreateWithProcessIdForUser(This, user, capabilityName, pid, result) \
    ((This)->lpVtbl->CreateWithProcessIdForUser(This, user, capabilityName, pid, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthorization_CAppCapabilityAccess_CIAppCapabilityStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Security.Authorization.AppCapabilityAccess.AppCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapability ** Default Interface **
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapability2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapability_DEFINED
#define RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authorization_AppCapabilityAccess_AppCapability[] = L"Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authorization.AppCapabilityAccess.IAppCapabilityAccessChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authorization_AppCapabilityAccess_AppCapabilityAccessChangedEventArgs[] = L"Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esecurity2Eauthorization2Eappcapabilityaccess_p_h__

#endif // __windows2Esecurity2Eauthorization2Eappcapabilityaccess_h__
