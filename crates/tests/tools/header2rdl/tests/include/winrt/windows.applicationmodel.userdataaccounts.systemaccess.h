
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
#ifndef __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_h__
#define __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_h__
#ifndef __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_p_h__
#define __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_p_h__


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
#include "Windows.Security.Credentials.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    interface IDeviceAccountConfiguration;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    interface IDeviceAccountConfiguration2;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2 ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration2

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    interface IUserDataAccountSystemAccessManagerStatics;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IUserDataAccountSystemAccessManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    interface IUserDataAccountSystemAccessManagerStatics2;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2 ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IUserDataAccountSystemAccessManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    class DeviceAccountConfiguration;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("469859f3-6b7b-5399-8a8c-fe615b95ae07"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*, ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cbee2c48-e3ed-5ebd-a4ae-56583388a49a"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*, ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f92b529-119b-575a-a419-3904b4e41af2"))
IAsyncOperation<__FIVectorView_1_HSTRING*> : IAsyncOperation_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_HSTRING*> __FIAsyncOperation_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c7899be-5f2e-5bf3-ade5-ad98b772c7cd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("51705a87-8dcb-5971-8d6b-ca8ae6a955ad"))
IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*, ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t;
#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a9c3ea6e-9dd9-52fe-9d27-f9e4dedd4d3f"))
IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*, ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountConfiguration*> __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t;
#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_USE */

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
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    typedef enum DeviceAccountAuthenticationType : int DeviceAccountAuthenticationType;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    typedef enum DeviceAccountIconId : int DeviceAccountIconId;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    typedef enum DeviceAccountMailAgeFilter : int DeviceAccountMailAgeFilter;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    typedef enum DeviceAccountServerType : int DeviceAccountServerType;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    typedef enum DeviceAccountSyncScheduleKind : int DeviceAccountSyncScheduleKind;
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    enum DeviceAccountAuthenticationType : int
                    {
                        DeviceAccountAuthenticationType_Basic = 0,
                        DeviceAccountAuthenticationType_OAuth = 1,
                        DeviceAccountAuthenticationType_SingleSignOn = 2,
                    };
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    enum DeviceAccountIconId : int
                    {
                        DeviceAccountIconId_Exchange = 0,
                        DeviceAccountIconId_Msa = 1,
                        DeviceAccountIconId_Outlook = 2,
                        DeviceAccountIconId_Generic = 3,
                    };
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    enum DeviceAccountMailAgeFilter : int
                    {
                        DeviceAccountMailAgeFilter_All = 0,
                        DeviceAccountMailAgeFilter_Last1Day = 1,
                        DeviceAccountMailAgeFilter_Last3Days = 2,
                        DeviceAccountMailAgeFilter_Last7Days = 3,
                        DeviceAccountMailAgeFilter_Last14Days = 4,
                        DeviceAccountMailAgeFilter_Last30Days = 5,
                        DeviceAccountMailAgeFilter_Last90Days = 6,
                    };
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    enum DeviceAccountServerType : int
                    {
                        DeviceAccountServerType_Exchange = 0,
                        DeviceAccountServerType_Pop = 1,
                        DeviceAccountServerType_Imap = 2,
                    };
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    enum DeviceAccountSyncScheduleKind : int
                    {
                        DeviceAccountSyncScheduleKind_Manual = 0,
                        DeviceAccountSyncScheduleKind_Every15Minutes = 1,
                        DeviceAccountSyncScheduleKind_Every30Minutes = 2,
                        DeviceAccountSyncScheduleKind_Every60Minutes = 3,
                        DeviceAccountSyncScheduleKind_Every2Hours = 4,
                        DeviceAccountSyncScheduleKind_Daily = 5,
                        DeviceAccountSyncScheduleKind_AsItemsArrive = 6,
                    };
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IDeviceAccountConfiguration[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    MIDL_INTERFACE("ad0123a3-fbdc-4d1b-be43-5a27ea4a1b63")
                    IDeviceAccountConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AccountName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AccountName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceAccountTypeId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DeviceAccountTypeId(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServerType(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountServerType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ServerType(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountServerType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailAddress(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EmailAddress(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Domain(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Domain(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailSyncEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EmailSyncEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContactsSyncEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContactsSyncEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CalendarSyncEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CalendarSyncEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerAddress(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerAddress(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerPort(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerPort(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerRequiresSsl(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerRequiresSsl(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerUsername(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerUsername(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerAddress(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerAddress(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerPort(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerPort(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerRequiresSsl(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerRequiresSsl(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerUsername(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerUsername(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDeviceAccountConfiguration = __uuidof(IDeviceAccountConfiguration);
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IDeviceAccountConfiguration2[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    MIDL_INTERFACE("f2b2e5a6-728d-4a4a-8945-2bf8580136de")
                    IDeviceAccountConfiguration2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerCredential(
                            ABI::Windows::Security::Credentials::IPasswordCredential* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OAuthRefreshToken(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OAuthRefreshToken(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsExternallyManaged(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsExternallyManaged(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccountIconId(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountIconId* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AccountIconId(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountIconId value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthenticationType(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountAuthenticationType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AuthenticationType(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountAuthenticationType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSsoAuthenticationSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SsoAccountId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SsoAccountId(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AlwaysDownloadFullMessage(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AlwaysDownloadFullMessage(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DoesPolicyAllowMailSync(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MailAgeFilter(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountMailAgeFilter* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MailAgeFilter(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountMailAgeFilter value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsClientAuthenticationCertificateRequired(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsClientAuthenticationCertificateRequired(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutoSelectAuthenticationCertificate(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AutoSelectAuthenticationCertificate(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthenticationCertificateId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AuthenticationCertificateId(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CardDavSyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CardDavSyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CalDavSyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CalDavSyncScheduleKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::DeviceAccountSyncScheduleKind value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CardDavServerUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CardDavServerUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CardDavRequiresSsl(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CardDavRequiresSsl(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CalDavServerUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CalDavServerUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CalDavRequiresSsl(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CalDavRequiresSsl(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WasModifiedByUser(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WasModifiedByUser(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WasIncomingServerCertificateHashConfirmed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WasIncomingServerCertificateHashConfirmed(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IncomingServerCertificateHash(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IncomingServerCertificateHash(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsOutgoingServerAuthenticationRequired(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsOutgoingServerAuthenticationRequired(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsOutgoingServerAuthenticationEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsOutgoingServerAuthenticationEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WasOutgoingServerCertificateHashConfirmed(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_WasOutgoingServerCertificateHashConfirmed(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OutgoingServerCertificateHash(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OutgoingServerCertificateHash(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSyncScheduleManagedBySystem(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsSyncScheduleManagedBySystem(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDeviceAccountConfiguration2 = __uuidof(IDeviceAccountConfiguration2);
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IUserDataAccountSystemAccessManagerStatics[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    MIDL_INTERFACE("9d6b11b9-cbe5-45f5-822b-c267b81dbdb6")
                    IUserDataAccountSystemAccessManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddAndShowDeviceAccountsAsync(
                            __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* accounts,
                            __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountSystemAccessManagerStatics = __uuidof(IUserDataAccountSystemAccessManagerStatics);
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IUserDataAccountSystemAccessManagerStatics2[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace SystemAccess {
                    MIDL_INTERFACE("943f854d-4b4e-439f-83d3-979b27c05ac7")
                    IUserDataAccountSystemAccessManagerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SuppressLocalAccountWithAccountAsync(
                            HSTRING userDataAccountId,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateDeviceAccountAsync(
                            ABI::Windows::ApplicationModel::UserDataAccounts::SystemAccess::IDeviceAccountConfiguration* account,
                            __FIAsyncOperation_1_HSTRING** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeleteDeviceAccountAsync(
                            HSTRING accountId,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceAccountConfigurationAsync(
                            HSTRING accountId,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountSystemAccessManagerStatics2 = __uuidof(IUserDataAccountSystemAccessManagerStatics2);
                } /* SystemAccess */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_HSTRING __FIAsyncOperation_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

typedef struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration;

typedef struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* This,
        __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountAuthenticationType __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountAuthenticationType;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountIconId __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountIconId;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountMailAgeFilter __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountMailAgeFilter;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountServerType __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountServerType;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind;

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountAuthenticationType
{
    DeviceAccountAuthenticationType_Basic = 0,
    DeviceAccountAuthenticationType_OAuth = 1,
    DeviceAccountAuthenticationType_SingleSignOn = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountIconId
{
    DeviceAccountIconId_Exchange = 0,
    DeviceAccountIconId_Msa = 1,
    DeviceAccountIconId_Outlook = 2,
    DeviceAccountIconId_Generic = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountMailAgeFilter
{
    DeviceAccountMailAgeFilter_All = 0,
    DeviceAccountMailAgeFilter_Last1Day = 1,
    DeviceAccountMailAgeFilter_Last3Days = 2,
    DeviceAccountMailAgeFilter_Last7Days = 3,
    DeviceAccountMailAgeFilter_Last14Days = 4,
    DeviceAccountMailAgeFilter_Last30Days = 5,
    DeviceAccountMailAgeFilter_Last90Days = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountServerType
{
    DeviceAccountServerType_Exchange = 0,
    DeviceAccountServerType_Pop = 1,
    DeviceAccountServerType_Imap = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind
{
    DeviceAccountSyncScheduleKind_Manual = 0,
    DeviceAccountSyncScheduleKind_Every15Minutes = 1,
    DeviceAccountSyncScheduleKind_Every30Minutes = 2,
    DeviceAccountSyncScheduleKind_Every60Minutes = 3,
    DeviceAccountSyncScheduleKind_Every2Hours = 4,
    DeviceAccountSyncScheduleKind_Daily = 5,
    DeviceAccountSyncScheduleKind_AsItemsArrive = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IDeviceAccountConfiguration[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AccountName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AccountName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAccountTypeId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DeviceAccountTypeId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServerType)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountServerType* value);
    HRESULT (STDMETHODCALLTYPE* put_ServerType)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountServerType value);
    HRESULT (STDMETHODCALLTYPE* get_EmailAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_EmailAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Domain)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Domain)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_EmailSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_EmailSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ContactsSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactsSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CalendarSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CalendarSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerPort)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerPort)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerUsername)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerUsername)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerAddress)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerPort)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerPort)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerUsername)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerUsername)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_AccountName(This, value) \
    ((This)->lpVtbl->get_AccountName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_AccountName(This, value) \
    ((This)->lpVtbl->put_AccountName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_DeviceAccountTypeId(This, value) \
    ((This)->lpVtbl->get_DeviceAccountTypeId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_DeviceAccountTypeId(This, value) \
    ((This)->lpVtbl->put_DeviceAccountTypeId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_ServerType(This, value) \
    ((This)->lpVtbl->get_ServerType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_ServerType(This, value) \
    ((This)->lpVtbl->put_ServerType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_EmailAddress(This, value) \
    ((This)->lpVtbl->get_EmailAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_EmailAddress(This, value) \
    ((This)->lpVtbl->put_EmailAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_Domain(This, value) \
    ((This)->lpVtbl->get_Domain(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_Domain(This, value) \
    ((This)->lpVtbl->put_Domain(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_EmailSyncEnabled(This, value) \
    ((This)->lpVtbl->get_EmailSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_EmailSyncEnabled(This, value) \
    ((This)->lpVtbl->put_EmailSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_ContactsSyncEnabled(This, value) \
    ((This)->lpVtbl->get_ContactsSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_ContactsSyncEnabled(This, value) \
    ((This)->lpVtbl->put_ContactsSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_CalendarSyncEnabled(This, value) \
    ((This)->lpVtbl->get_CalendarSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_CalendarSyncEnabled(This, value) \
    ((This)->lpVtbl->put_CalendarSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_IncomingServerAddress(This, value) \
    ((This)->lpVtbl->get_IncomingServerAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_IncomingServerAddress(This, value) \
    ((This)->lpVtbl->put_IncomingServerAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_IncomingServerPort(This, value) \
    ((This)->lpVtbl->get_IncomingServerPort(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_IncomingServerPort(This, value) \
    ((This)->lpVtbl->put_IncomingServerPort(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_IncomingServerRequiresSsl(This, value) \
    ((This)->lpVtbl->get_IncomingServerRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_IncomingServerRequiresSsl(This, value) \
    ((This)->lpVtbl->put_IncomingServerRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_IncomingServerUsername(This, value) \
    ((This)->lpVtbl->get_IncomingServerUsername(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_IncomingServerUsername(This, value) \
    ((This)->lpVtbl->put_IncomingServerUsername(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_OutgoingServerAddress(This, value) \
    ((This)->lpVtbl->get_OutgoingServerAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_OutgoingServerAddress(This, value) \
    ((This)->lpVtbl->put_OutgoingServerAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_OutgoingServerPort(This, value) \
    ((This)->lpVtbl->get_OutgoingServerPort(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_OutgoingServerPort(This, value) \
    ((This)->lpVtbl->put_OutgoingServerPort(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_OutgoingServerRequiresSsl(This, value) \
    ((This)->lpVtbl->get_OutgoingServerRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_OutgoingServerRequiresSsl(This, value) \
    ((This)->lpVtbl->put_OutgoingServerRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_get_OutgoingServerUsername(This, value) \
    ((This)->lpVtbl->get_OutgoingServerUsername(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_put_OutgoingServerUsername(This, value) \
    ((This)->lpVtbl->put_OutgoingServerUsername(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IDeviceAccountConfiguration2[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerCredential)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerCredential)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerCredential)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerCredential)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* value);
    HRESULT (STDMETHODCALLTYPE* get_OAuthRefreshToken)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OAuthRefreshToken)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsExternallyManaged)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsExternallyManaged)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AccountIconId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountIconId* value);
    HRESULT (STDMETHODCALLTYPE* put_AccountIconId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountIconId value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticationType)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountAuthenticationType* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthenticationType)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountAuthenticationType value);
    HRESULT (STDMETHODCALLTYPE* get_IsSsoAuthenticationSupported)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SsoAccountId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SsoAccountId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AlwaysDownloadFullMessage)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AlwaysDownloadFullMessage)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DoesPolicyAllowMailSync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind* value);
    HRESULT (STDMETHODCALLTYPE* put_SyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind value);
    HRESULT (STDMETHODCALLTYPE* get_MailAgeFilter)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountMailAgeFilter* value);
    HRESULT (STDMETHODCALLTYPE* put_MailAgeFilter)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountMailAgeFilter value);
    HRESULT (STDMETHODCALLTYPE* get_IsClientAuthenticationCertificateRequired)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsClientAuthenticationCertificateRequired)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AutoSelectAuthenticationCertificate)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoSelectAuthenticationCertificate)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticationCertificateId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthenticationCertificateId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CardDavSyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind* value);
    HRESULT (STDMETHODCALLTYPE* put_CardDavSyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind value);
    HRESULT (STDMETHODCALLTYPE* get_CalDavSyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind* value);
    HRESULT (STDMETHODCALLTYPE* put_CalDavSyncScheduleKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CDeviceAccountSyncScheduleKind value);
    HRESULT (STDMETHODCALLTYPE* get_CardDavServerUrl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_CardDavServerUrl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_CardDavRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CardDavRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CalDavServerUrl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_CalDavServerUrl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_CalDavRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CalDavRequiresSsl)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_WasModifiedByUser)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_WasModifiedByUser)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_WasIncomingServerCertificateHashConfirmed)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_WasIncomingServerCertificateHashConfirmed)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncomingServerCertificateHash)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_IncomingServerCertificateHash)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsOutgoingServerAuthenticationRequired)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsOutgoingServerAuthenticationRequired)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsOutgoingServerAuthenticationEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsOutgoingServerAuthenticationEnabled)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_WasOutgoingServerCertificateHashConfirmed)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_WasOutgoingServerCertificateHashConfirmed)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingServerCertificateHash)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingServerCertificateHash)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsSyncScheduleManagedBySystem)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSyncScheduleManagedBySystem)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IncomingServerCredential(This, value) \
    ((This)->lpVtbl->get_IncomingServerCredential(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IncomingServerCredential(This, value) \
    ((This)->lpVtbl->put_IncomingServerCredential(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_OutgoingServerCredential(This, value) \
    ((This)->lpVtbl->get_OutgoingServerCredential(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_OutgoingServerCredential(This, value) \
    ((This)->lpVtbl->put_OutgoingServerCredential(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_OAuthRefreshToken(This, value) \
    ((This)->lpVtbl->get_OAuthRefreshToken(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_OAuthRefreshToken(This, value) \
    ((This)->lpVtbl->put_OAuthRefreshToken(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsExternallyManaged(This, value) \
    ((This)->lpVtbl->get_IsExternallyManaged(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IsExternallyManaged(This, value) \
    ((This)->lpVtbl->put_IsExternallyManaged(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_AccountIconId(This, value) \
    ((This)->lpVtbl->get_AccountIconId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_AccountIconId(This, value) \
    ((This)->lpVtbl->put_AccountIconId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_AuthenticationType(This, value) \
    ((This)->lpVtbl->get_AuthenticationType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_AuthenticationType(This, value) \
    ((This)->lpVtbl->put_AuthenticationType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsSsoAuthenticationSupported(This, value) \
    ((This)->lpVtbl->get_IsSsoAuthenticationSupported(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_SsoAccountId(This, value) \
    ((This)->lpVtbl->get_SsoAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_SsoAccountId(This, value) \
    ((This)->lpVtbl->put_SsoAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_AlwaysDownloadFullMessage(This, value) \
    ((This)->lpVtbl->get_AlwaysDownloadFullMessage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_AlwaysDownloadFullMessage(This, value) \
    ((This)->lpVtbl->put_AlwaysDownloadFullMessage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_DoesPolicyAllowMailSync(This, value) \
    ((This)->lpVtbl->get_DoesPolicyAllowMailSync(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_SyncScheduleKind(This, value) \
    ((This)->lpVtbl->get_SyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_SyncScheduleKind(This, value) \
    ((This)->lpVtbl->put_SyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_MailAgeFilter(This, value) \
    ((This)->lpVtbl->get_MailAgeFilter(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_MailAgeFilter(This, value) \
    ((This)->lpVtbl->put_MailAgeFilter(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsClientAuthenticationCertificateRequired(This, value) \
    ((This)->lpVtbl->get_IsClientAuthenticationCertificateRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IsClientAuthenticationCertificateRequired(This, value) \
    ((This)->lpVtbl->put_IsClientAuthenticationCertificateRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_AutoSelectAuthenticationCertificate(This, value) \
    ((This)->lpVtbl->get_AutoSelectAuthenticationCertificate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_AutoSelectAuthenticationCertificate(This, value) \
    ((This)->lpVtbl->put_AutoSelectAuthenticationCertificate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_AuthenticationCertificateId(This, value) \
    ((This)->lpVtbl->get_AuthenticationCertificateId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_AuthenticationCertificateId(This, value) \
    ((This)->lpVtbl->put_AuthenticationCertificateId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CardDavSyncScheduleKind(This, value) \
    ((This)->lpVtbl->get_CardDavSyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CardDavSyncScheduleKind(This, value) \
    ((This)->lpVtbl->put_CardDavSyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CalDavSyncScheduleKind(This, value) \
    ((This)->lpVtbl->get_CalDavSyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CalDavSyncScheduleKind(This, value) \
    ((This)->lpVtbl->put_CalDavSyncScheduleKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CardDavServerUrl(This, value) \
    ((This)->lpVtbl->get_CardDavServerUrl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CardDavServerUrl(This, value) \
    ((This)->lpVtbl->put_CardDavServerUrl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CardDavRequiresSsl(This, value) \
    ((This)->lpVtbl->get_CardDavRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CardDavRequiresSsl(This, value) \
    ((This)->lpVtbl->put_CardDavRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CalDavServerUrl(This, value) \
    ((This)->lpVtbl->get_CalDavServerUrl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CalDavServerUrl(This, value) \
    ((This)->lpVtbl->put_CalDavServerUrl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_CalDavRequiresSsl(This, value) \
    ((This)->lpVtbl->get_CalDavRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_CalDavRequiresSsl(This, value) \
    ((This)->lpVtbl->put_CalDavRequiresSsl(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_WasModifiedByUser(This, value) \
    ((This)->lpVtbl->get_WasModifiedByUser(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_WasModifiedByUser(This, value) \
    ((This)->lpVtbl->put_WasModifiedByUser(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_WasIncomingServerCertificateHashConfirmed(This, value) \
    ((This)->lpVtbl->get_WasIncomingServerCertificateHashConfirmed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_WasIncomingServerCertificateHashConfirmed(This, value) \
    ((This)->lpVtbl->put_WasIncomingServerCertificateHashConfirmed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IncomingServerCertificateHash(This, value) \
    ((This)->lpVtbl->get_IncomingServerCertificateHash(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IncomingServerCertificateHash(This, value) \
    ((This)->lpVtbl->put_IncomingServerCertificateHash(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsOutgoingServerAuthenticationRequired(This, value) \
    ((This)->lpVtbl->get_IsOutgoingServerAuthenticationRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IsOutgoingServerAuthenticationRequired(This, value) \
    ((This)->lpVtbl->put_IsOutgoingServerAuthenticationRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsOutgoingServerAuthenticationEnabled(This, value) \
    ((This)->lpVtbl->get_IsOutgoingServerAuthenticationEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IsOutgoingServerAuthenticationEnabled(This, value) \
    ((This)->lpVtbl->put_IsOutgoingServerAuthenticationEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_WasOutgoingServerCertificateHashConfirmed(This, value) \
    ((This)->lpVtbl->get_WasOutgoingServerCertificateHashConfirmed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_WasOutgoingServerCertificateHashConfirmed(This, value) \
    ((This)->lpVtbl->put_WasOutgoingServerCertificateHashConfirmed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_OutgoingServerCertificateHash(This, value) \
    ((This)->lpVtbl->get_OutgoingServerCertificateHash(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_OutgoingServerCertificateHash(This, value) \
    ((This)->lpVtbl->put_OutgoingServerCertificateHash(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_get_IsSyncScheduleManagedBySystem(This, value) \
    ((This)->lpVtbl->get_IsSyncScheduleManagedBySystem(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_put_IsSyncScheduleManagedBySystem(This, value) \
    ((This)->lpVtbl->put_IsSyncScheduleManagedBySystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IUserDataAccountSystemAccessManagerStatics[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddAndShowDeviceAccountsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics* This,
        __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration* accounts,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_AddAndShowDeviceAccountsAsync(This, accounts, result) \
    ((This)->lpVtbl->AddAndShowDeviceAccountsAsync(This, accounts, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_SystemAccess_IUserDataAccountSystemAccessManagerStatics2[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SuppressLocalAccountWithAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        HSTRING userDataAccountId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIDeviceAccountConfiguration* account,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* DeleteDeviceAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        HSTRING accountId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceAccountConfigurationAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2* This,
        HSTRING accountId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CSystemAccess__CDeviceAccountConfiguration** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_SuppressLocalAccountWithAccountAsync(This, userDataAccountId, result) \
    ((This)->lpVtbl->SuppressLocalAccountWithAccountAsync(This, userDataAccountId, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_CreateDeviceAccountAsync(This, account, result) \
    ((This)->lpVtbl->CreateDeviceAccountAsync(This, account, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_DeleteDeviceAccountAsync(This, accountId, result) \
    ((This)->lpVtbl->DeleteDeviceAccountAsync(This, accountId, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_GetDeviceAccountConfigurationAsync(This, accountId, result) \
    ((This)->lpVtbl->GetDeviceAccountConfigurationAsync(This, accountId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CSystemAccess_CIUserDataAccountSystemAccessManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_SystemAccess_DeviceAccountConfiguration[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_SystemAccess_UserDataAccountSystemAccessManager[] = L"Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
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
#endif // __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_p_h__

#endif // __windows2Eapplicationmodel2Euserdataaccounts2Esystemaccess_h__
