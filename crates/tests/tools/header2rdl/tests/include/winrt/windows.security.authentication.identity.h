
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
#ifndef __windows2Esecurity2Eauthentication2Eidentity_h__
#define __windows2Esecurity2Eauthentication2Eidentity_h__
#ifndef __windows2Esecurity2Eauthentication2Eidentity_p_h__
#define __windows2Esecurity2Eauthentication2Eidentity_p_h__


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
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    interface IEnterpriseKeyCredentialRegistrationInfo;
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationInfo

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    interface IEnterpriseKeyCredentialRegistrationManager;
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationManager

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    interface IEnterpriseKeyCredentialRegistrationManagerStatics;
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationManagerStatics

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    class EnterpriseKeyCredentialRegistrationInfo;
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#define DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e3497259-1737-5fad-803b-9d2d29273e3b"))
IIterator<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*, ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t;
#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#define DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e7eea796-77f9-5473-a913-734ea0e3ff46"))
IIterable<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*, ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t;
#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3b631cbb-63f8-5eff-8815-69c822c09ce1"))
IVectorView<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*, ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Authentication::Identity::EnterpriseKeyCredentialRegistrationInfo*> __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t;
#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0bd64c2f-8b1d-56d4-a707-fab5315e7278"))
IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*> __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67746c40-ade0-5981-ae23-104891748853"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    class EnterpriseKeyCredentialRegistrationManager;
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationInfo[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    MIDL_INTERFACE("38321acc-672b-4823-b603-6b3c753daf97")
                    IEnterpriseKeyCredentialRegistrationInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TenantId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TenantName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEnterpriseKeyCredentialRegistrationInfo = __uuidof(IEnterpriseKeyCredentialRegistrationInfo);
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationManager[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    MIDL_INTERFACE("83f3be3f-a25f-4cba-bb8e-bdc32d03c297")
                    IEnterpriseKeyCredentialRegistrationManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetRegistrationsAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEnterpriseKeyCredentialRegistrationManager = __uuidof(IEnterpriseKeyCredentialRegistrationManager);
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationManagerStatics[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Authentication {
                namespace Identity {
                    MIDL_INTERFACE("77b85e9e-acf4-4bc0-bac2-40bb46efbb3f")
                    IEnterpriseKeyCredentialRegistrationManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::Security::Authentication::Identity::IEnterpriseKeyCredentialRegistrationManager** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEnterpriseKeyCredentialRegistrationManagerStatics = __uuidof(IEnterpriseKeyCredentialRegistrationManagerStatics);
                } /* Identity */
            } /* Authentication */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo[] = L"Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager[] = L"Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

typedef struct __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

typedef struct __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __FIIterator_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

typedef struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationInfo[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TenantId)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TenantName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyId)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_get_TenantId(This, value) \
    ((This)->lpVtbl->get_TenantId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_get_TenantName(This, value) \
    ((This)->lpVtbl->get_TenantName(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_get_KeyId(This, value) \
    ((This)->lpVtbl->get_KeyId(This, value))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_get_KeyName(This, value) \
    ((This)->lpVtbl->get_KeyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationManager[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetRegistrationsAsync)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CSecurity__CAuthentication__CIdentity__CEnterpriseKeyCredentialRegistrationInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_GetRegistrationsAsync(This, value) \
    ((This)->lpVtbl->GetRegistrationsAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Authentication_Identity_IEnterpriseKeyCredentialRegistrationManagerStatics[] = L"Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics* This,
        __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CAuthentication_CIdentity_CIEnterpriseKeyCredentialRegistrationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationInfo[] = L"Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Authentication.Identity.IEnterpriseKeyCredentialRegistrationManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager_DEFINED
#define RUNTIMECLASS_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Authentication_Identity_EnterpriseKeyCredentialRegistrationManager[] = L"Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
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
#endif // __windows2Esecurity2Eauthentication2Eidentity_p_h__

#endif // __windows2Esecurity2Eauthentication2Eidentity_h__
