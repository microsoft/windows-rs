
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
#ifndef __windows2Eapplicationmodel2Euserdataaccounts_h__
#define __windows2Eapplicationmodel2Euserdataaccounts_h__
#ifndef __windows2Eapplicationmodel2Euserdataaccounts_p_h__
#define __windows2Eapplicationmodel2Euserdataaccounts_p_h__


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
#include "Windows.ApplicationModel.Appointments.h"
#include "Windows.ApplicationModel.Contacts.h"
#include "Windows.ApplicationModel.Email.h"
#include "Windows.ApplicationModel.UserDataTasks.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccount;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccount2;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount2

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccount3;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount3

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccount4;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount4

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountManagerForUser;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountManagerForUser

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountManagerStatics;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountManagerStatics2;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountStore;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountStore2;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore2

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountStore3;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3 ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore3

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                interface IUserDataAccountStoreChangedEventArgs;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStoreChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__

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
                class UserDataAccount;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f0aeb2af-a69b-5caa-a283-32e697a65d31"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab92e426-2ac6-5ffb-88ca-cbdd91df3927"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                class UserDataAccountStore;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("06b68f1e-9937-5296-a55e-d43dd8a7545c"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("264c2ca9-29b4-5035-b460-8c8d0e8fbcd9"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentCalendar;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendar;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f40a879d-b6b7-5f92-beb1-6a8e7ce54120"))
IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3fb010b-b692-5130-9d16-2cfdabcb6dec"))
IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("774512d3-a564-5f8d-b104-3ec8f2a1104f"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d26021ac-99b4-5c40-b0d2-d6835d2e9202"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f478469b-9777-553d-beab-1bb5eee1ca8b"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactAnnotationList;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactAnnotationList;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList ABI::Windows::ApplicationModel::Contacts::IContactAnnotationList

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3d7b4ad2-bb54-5e21-a92e-6b80264ccc50"))
IIterator<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*, ABI::Windows::ApplicationModel::Contacts::IContactAnnotationList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Contacts.ContactAnnotationList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t;
#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ae095a89-8b8b-5b8a-8f28-555aa490ad87"))
IIterable<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*, ABI::Windows::ApplicationModel::Contacts::IContactAnnotationList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Contacts.ContactAnnotationList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t;
#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3bf71d9d-2037-5e08-a7e1-b937cf74bbd9"))
IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*, ABI::Windows::ApplicationModel::Contacts::IContactAnnotationList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactAnnotationList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactAnnotationList*> __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t;
#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a549e2bf-9e24-5352-923d-ff184f96acbb"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactAnnotationList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dd802265-ec6f-5573-a09e-02b1f18a8a51"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactAnnotationList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactGroup;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactGroup;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup ABI::Windows::ApplicationModel::Contacts::IContactGroup

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("86dac457-7314-5044-8108-75290c36dd2b"))
IIterator<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactGroup*, ABI::Windows::ApplicationModel::Contacts::IContactGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Contacts.ContactGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_t;
#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9db1688a-f7e0-5059-989e-d1453056d73a"))
IIterable<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactGroup*, ABI::Windows::ApplicationModel::Contacts::IContactGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Contacts.ContactGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_t;
#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("124493aa-1617-5d0a-a5b6-4d1156fa95e5"))
IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactGroup*, ABI::Windows::ApplicationModel::Contacts::IContactGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactGroup*> __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t;
#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f78d6a56-6e40-5f68-99de-b0ec119ccab2"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd49aed6-182c-5847-9467-f750d1d029bc"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactList;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactList;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList ABI::Windows::ApplicationModel::Contacts::IContactList

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee69382d-887d-5964-83b4-47ee9ded6f05"))
IIterator<ABI::Windows::ApplicationModel::Contacts::ContactList*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactList*, ABI::Windows::ApplicationModel::Contacts::IContactList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Contacts.ContactList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Contacts::ContactList*> __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_t;
#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f9b0782c-bfe9-564e-bca2-97235fd64463"))
IIterable<ABI::Windows::ApplicationModel::Contacts::ContactList*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactList*, ABI::Windows::ApplicationModel::Contacts::IContactList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Contacts.ContactList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Contacts::ContactList*> __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_t;
#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a55c7dcf-e3c9-5afa-8667-ca68230d7724"))
IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactList*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::ContactList*, ABI::Windows::ApplicationModel::Contacts::IContactList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Contacts::ContactList*> __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t;
#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("79fd4783-5cc9-571b-bc42-e2e2f8ddf967"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eaafaa0f-472b-5cb1-ad7a-ad9fd505e2c5"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailMailbox;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailMailbox;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox ABI::Windows::ApplicationModel::Email::IEmailMailbox

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("312f8d4a-0e85-566e-b7aa-dbd40a0343ff"))
IIterator<ABI::Windows::ApplicationModel::Email::EmailMailbox*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailMailbox*, ABI::Windows::ApplicationModel::Email::IEmailMailbox*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Email.EmailMailbox>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Email::EmailMailbox*> __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t;
#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8fc76b2e-f800-5f1d-a023-de47e5f306e0"))
IIterable<ABI::Windows::ApplicationModel::Email::EmailMailbox*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailMailbox*, ABI::Windows::ApplicationModel::Email::IEmailMailbox*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Email.EmailMailbox>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Email::EmailMailbox*> __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t;
#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d4816e16-43c9-5b63-a444-6189acb56f6e"))
IVectorView<ABI::Windows::ApplicationModel::Email::EmailMailbox*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailMailbox*, ABI::Windows::ApplicationModel::Email::IEmailMailbox*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Email.EmailMailbox>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Email::EmailMailbox*> __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t;
#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5bed07ec-f5d5-5b25-b067-9b22a4a762ea"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Email.EmailMailbox>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("478ddb6a-e122-5832-8263-f3d90a1f5377"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Email.EmailMailbox>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e38246ab-df01-5d16-923a-17c88cdbac29"))
IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1bc7f1ff-26b4-5440-9629-cb4cd8c87987"))
IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("65f4f73e-3271-5999-93d8-5a3d78be13a6"))
IVectorView<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccount*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccount*> __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8461a475-5f2f-5a03-b9f8-01e1324def3b"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dc12096d-7e54-54a2-ab99-280f30f0ff81"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataAccounts.UserDataAccount>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskList;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskList;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e989bcb-9d7c-512f-89da-fdb75532d665"))
IIterator<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c4d63bc-cffb-50d2-8a82-a9aefad651d3"))
IIterable<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0910e3ae-2075-5486-866f-f1590ade3616"))
IVectorView<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2a4e06ee-9030-5ce6-ad7f-cf551795765d"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4aba8568-5231-526c-b2e4-805006b8ef2f"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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



#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                class UserDataAccountStoreChangedEventArgs;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7b27aec8-0690-5063-beb0-d9e2eb1a1201"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*, ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStoreChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStore*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStoreChangedEventArgs*, ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountStoreChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore, Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStore*, ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStoreChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace UserDataAccounts {
                typedef enum UserDataAccountContentKinds : unsigned int UserDataAccountContentKinds;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                typedef enum UserDataAccountOtherAppReadAccess : int UserDataAccountOtherAppReadAccess;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                typedef enum UserDataAccountStoreAccessType : int UserDataAccountStoreAccessType;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                class UserDataAccountManagerForUser;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                enum UserDataAccountContentKinds : unsigned int
                {
                    UserDataAccountContentKinds_Email = 0x1,
                    UserDataAccountContentKinds_Contact = 0x2,
                    UserDataAccountContentKinds_Appointment = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(UserDataAccountContentKinds)
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                enum UserDataAccountOtherAppReadAccess : int
                {
                    UserDataAccountOtherAppReadAccess_SystemOnly = 0,
                    UserDataAccountOtherAppReadAccess_Full = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    UserDataAccountOtherAppReadAccess_None = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                enum UserDataAccountStoreAccessType : int
                {
                    UserDataAccountStoreAccessType_AllAccountsReadOnly = 0,
                    UserDataAccountStoreAccessType_AppAccountsReadWrite = 1,
                };
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("b9c4367e-b348-4910-be94-4ad4bba6dea7")
                IUserDataAccount : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UserDisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountOtherAppReadAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountOtherAppReadAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceAccountTypeId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentCalendarsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindEmailMailboxesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindContactListsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindContactAnnotationListsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccount = __uuidof(IUserDataAccount);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("078cd89f-de82-404b-8195-c8a3ac198f60")
                IUserDataAccount2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EnterpriseId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsProtectedUnderLock(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccount2 = __uuidof(IUserDataAccount2);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount3[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("01533845-6c43-4286-9d69-3e1709a1f266")
                IUserDataAccount3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExplictReadAccessPackageFamilyNames(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccount3 = __uuidof(IUserDataAccount3);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount4[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("c4315210-eae5-4f0a-a8b2-1cca115e008f")
                IUserDataAccount4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanShowCreateContactGroup(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanShowCreateContactGroup(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderProperties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindUserDataTaskListsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindContactGroupsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowCreateContactGroupAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsProtectedUnderLock(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Icon(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccount4 = __uuidof(IUserDataAccount4);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerForUser[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("56a6e8db-db8f-41ab-a65f-8c5971aac982")
                IUserDataAccountManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStoreAccessType storeAccessType,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountManagerForUser = __uuidof(IUserDataAccountManagerForUser);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerStatics[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("0d9b89ea-1928-4a20-86d5-3c737f7dc3b0")
                IUserDataAccountManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountStoreAccessType storeAccessType,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAccountAsync(
                        ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountContentKinds contentKinds,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAccountSettingsAsync(
                        HSTRING id,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAccountErrorResolverAsync(
                        HSTRING id,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountManagerStatics = __uuidof(IUserDataAccountManagerStatics);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerStatics2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("6a3ded88-316b-435e-b534-f7d4b4b7dba6")
                IUserDataAccountManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::ApplicationModel::UserDataAccounts::IUserDataAccountManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountManagerStatics2 = __uuidof(IUserDataAccountManagerStatics2);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("2073b0ad-7d0a-4e76-bf45-2368f978a59a")
                IUserDataAccountStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAccountsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAccountAsync(
                        HSTRING id,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAccountAsync(
                        HSTRING userDisplayName,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountStore = __uuidof(IUserDataAccountStore);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("b1e0aef7-9560-4631-8af0-061d30161469")
                IUserDataAccountStore2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAccountWithPackageRelativeAppIdAsync(
                        HSTRING userDisplayName,
                        HSTRING packageRelativeAppId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StoreChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StoreChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountStore2 = __uuidof(IUserDataAccountStore2);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore3[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("8142c094-f3c9-478b-b117-6585bebb6789")
                IUserDataAccountStore3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync(
                        HSTRING userDisplayName,
                        HSTRING packageRelativeAppId,
                        HSTRING enterpriseId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountStore3 = __uuidof(IUserDataAccountStore3);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStoreChangedEventArgs[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                MIDL_INTERFACE("84e3e2e5-8820-4512-b1f6-2e035be1072c")
                IUserDataAccountStoreChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataAccountStoreChangedEventArgs = __uuidof(IUserDataAccountStoreChangedEventArgs);
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccount_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccount[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3 __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

typedef struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

typedef struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __FIIterator_1_Windows__CApplicationModel__CContacts__CContactAnnotationList** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactAnnotationList** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup;

typedef struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup;

typedef struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __FIIterator_1_Windows__CApplicationModel__CContacts__CContactGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CContacts__CContactList;

typedef struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactList* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CContacts__CContactListVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CContacts__CContactList;

typedef struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __FIIterator_1_Windows__CApplicationModel__CContacts__CContactList** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CContacts__CContactListVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactList** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

typedef struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

typedef struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailMailbox** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailbox** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailboxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountOtherAppReadAccess __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountOtherAppReadAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountStoreAccessType __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountStoreAccessType;

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds
{
    UserDataAccountContentKinds_Email = 0x1,
    UserDataAccountContentKinds_Contact = 0x2,
    UserDataAccountContentKinds_Appointment = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountOtherAppReadAccess
{
    UserDataAccountOtherAppReadAccess_SystemOnly = 0,
    UserDataAccountOtherAppReadAccess_Full = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    UserDataAccountOtherAppReadAccess_None = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountStoreAccessType
{
    UserDataAccountStoreAccessType_AllAccountsReadOnly = 0,
    UserDataAccountStoreAccessType_AppAccountsReadWrite = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserDisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_UserDisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountOtherAppReadAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountOtherAppReadAccess value);
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceAccountTypeId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentCalendarsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* FindEmailMailboxesAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailMailbox** result);
    HRESULT (STDMETHODCALLTYPE* FindContactListsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactList** result);
    HRESULT (STDMETHODCALLTYPE* FindContactAnnotationListsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactAnnotationList** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_UserDisplayName(This, value) \
    ((This)->lpVtbl->get_UserDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_put_UserDisplayName(This, value) \
    ((This)->lpVtbl->put_UserDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_put_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_DeviceAccountTypeId(This, value) \
    ((This)->lpVtbl->get_DeviceAccountTypeId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_SaveAsync(This, result) \
    ((This)->lpVtbl->SaveAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_DeleteAsync(This, result) \
    ((This)->lpVtbl->DeleteAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FindAppointmentCalendarsAsync(This, result) \
    ((This)->lpVtbl->FindAppointmentCalendarsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FindEmailMailboxesAsync(This, result) \
    ((This)->lpVtbl->FindEmailMailboxesAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FindContactListsAsync(This, result) \
    ((This)->lpVtbl->FindContactListsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_FindContactAnnotationListsAsync(This, result) \
    ((This)->lpVtbl->FindContactAnnotationListsAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EnterpriseId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsProtectedUnderLock)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_get_EnterpriseId(This, value) \
    ((This)->lpVtbl->get_EnterpriseId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_get_IsProtectedUnderLock(This, value) \
    ((This)->lpVtbl->get_IsProtectedUnderLock(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount3[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExplictReadAccessPackageFamilyNames)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_get_ExplictReadAccessPackageFamilyNames(This, value) \
    ((This)->lpVtbl->get_ExplictReadAccessPackageFamilyNames(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccount4[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanShowCreateContactGroup)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanShowCreateContactGroup)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderProperties)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* FindUserDataTaskListsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation);
    HRESULT (STDMETHODCALLTYPE* FindContactGroupsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactGroup** operation);
    HRESULT (STDMETHODCALLTYPE* TryShowCreateContactGroupAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* put_IsProtectedUnderLock)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* put_Icon)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_get_CanShowCreateContactGroup(This, value) \
    ((This)->lpVtbl->get_CanShowCreateContactGroup(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_put_CanShowCreateContactGroup(This, value) \
    ((This)->lpVtbl->put_CanShowCreateContactGroup(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_get_ProviderProperties(This, value) \
    ((This)->lpVtbl->get_ProviderProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FindUserDataTaskListsAsync(This, operation) \
    ((This)->lpVtbl->FindUserDataTaskListsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_FindContactGroupsAsync(This, operation) \
    ((This)->lpVtbl->FindContactGroupsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_TryShowCreateContactGroupAsync(This, operation) \
    ((This)->lpVtbl->TryShowCreateContactGroupAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_put_IsProtectedUnderLock(This, value) \
    ((This)->lpVtbl->put_IsProtectedUnderLock(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_put_Icon(This, value) \
    ((This)->lpVtbl->put_Icon(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccount4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerForUser[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountStoreAccessType storeAccessType,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore** result);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUserVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_RequestStoreAsync(This, storeAccessType, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, storeAccessType, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerStatics[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountStoreAccessType storeAccessType,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore** result);
    HRESULT (STDMETHODCALLTYPE* ShowAddAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds contentKinds,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowAccountSettingsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ShowAccountErrorResolverAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_RequestStoreAsync(This, storeAccessType, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, storeAccessType, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_ShowAddAccountAsync(This, contentKinds, result) \
    ((This)->lpVtbl->ShowAddAccountAsync(This, contentKinds, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_ShowAccountSettingsAsync(This, id, result) \
    ((This)->lpVtbl->ShowAccountSettingsAsync(This, id, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_ShowAccountErrorResolverAsync(This, id, result) \
    ((This)->lpVtbl->ShowAccountErrorResolverAsync(This, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountManagerStatics2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAccountsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* GetAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        HSTRING id,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* CreateAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore* This,
        HSTRING userDisplayName,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_FindAccountsAsync(This, result) \
    ((This)->lpVtbl->FindAccountsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_GetAccountAsync(This, id, result) \
    ((This)->lpVtbl->GetAccountAsync(This, id, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_CreateAccountAsync(This, userDisplayName, result) \
    ((This)->lpVtbl->CreateAccountAsync(This, userDisplayName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore2[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAccountWithPackageRelativeAppIdAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        HSTRING userDisplayName,
        HSTRING packageRelativeAppId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);
    HRESULT (STDMETHODCALLTYPE* add_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStore_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccountStoreChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_CreateAccountWithPackageRelativeAppIdAsync(This, userDisplayName, packageRelativeAppId, result) \
    ((This)->lpVtbl->CreateAccountWithPackageRelativeAppIdAsync(This, userDisplayName, packageRelativeAppId, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_add_StoreChanged(This, handler, token) \
    ((This)->lpVtbl->add_StoreChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_remove_StoreChanged(This, token) \
    ((This)->lpVtbl->remove_StoreChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStore3[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3* This,
        HSTRING userDisplayName,
        HSTRING packageRelativeAppId,
        HSTRING enterpriseId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataAccounts__CUserDataAccount** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync(This, userDisplayName, packageRelativeAppId, enterpriseId, result) \
    ((This)->lpVtbl->CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync(This, userDisplayName, packageRelativeAppId, enterpriseId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_IUserDataAccountStoreChangedEventArgs[] = L"Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CIUserDataAccountStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount2
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount3
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccount4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccount_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccount[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManager[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountManagerForUser[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore2
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStore[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.IUserDataAccountStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_UserDataAccountStoreChangedEventArgs[] = L"Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
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
#endif // __windows2Eapplicationmodel2Euserdataaccounts_p_h__

#endif // __windows2Eapplicationmodel2Euserdataaccounts_h__
