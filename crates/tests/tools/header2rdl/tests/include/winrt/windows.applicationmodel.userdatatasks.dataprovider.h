
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
#ifndef __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_h__
#define __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_h__
#ifndef __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_p_h__
#define __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_p_h__


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
#include "Windows.ApplicationModel.UserDataTasks.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskDataProviderConnection;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskDataProviderTriggerDetails;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListCompleteTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCompleteTaskRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListCompleteTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCompleteTaskRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListCreateOrUpdateTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCreateOrUpdateTaskRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListCreateOrUpdateTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCreateOrUpdateTaskRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListDeleteTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListDeleteTaskRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListDeleteTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListDeleteTaskRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListSkipOccurrenceRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSkipOccurrenceRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListSkipOccurrenceRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSkipOccurrenceRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListSyncManagerSyncRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSyncManagerSyncRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    interface IUserDataTaskListSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSyncManagerSyncRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskDataProviderConnection;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListCompleteTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6035f9f7-d4c5-5394-b0e3-5d606987ba47"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCompleteTaskRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCompleteTaskRequestEventArgs*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCompleteTaskRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection, Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCompleteTaskRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListCreateOrUpdateTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28adf45c-6807-5590-a7f1-934747937592"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCreateOrUpdateTaskRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCreateOrUpdateTaskRequestEventArgs*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCreateOrUpdateTaskRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection, Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListCreateOrUpdateTaskRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListDeleteTaskRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0b18e688-b269-5ca2-a8f3-d6d10f0fb320"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListDeleteTaskRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListDeleteTaskRequestEventArgs*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListDeleteTaskRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection, Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListDeleteTaskRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListSkipOccurrenceRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0887b178-c546-5ac8-ae10-3292ab5265bc"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSkipOccurrenceRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSkipOccurrenceRequestEventArgs*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSkipOccurrenceRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection, Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSkipOccurrenceRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b164e8ef-167c-5852-a792-0930b4001871"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSyncManagerSyncRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSyncManagerSyncRequestEventArgs*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSyncManagerSyncRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection, Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskDataProviderConnection*, ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::UserDataTaskListSyncManagerSyncRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTask;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTask;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListCompleteTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListCreateOrUpdateTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListDeleteTaskRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListSkipOccurrenceRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    class UserDataTaskListSyncManagerSyncRequest;
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskDataProviderConnection[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("9ff39d1d-a447-428b-afe9-e5402bdeb041")
                    IUserDataTaskDataProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_CreateOrUpdateTaskRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CreateOrUpdateTaskRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SyncRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SyncRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SkipOccurrenceRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SkipOccurrenceRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CompleteTaskRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CompleteTaskRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DeleteTaskRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DeleteTaskRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskDataProviderConnection = __uuidof(IUserDataTaskDataProviderConnection);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskDataProviderTriggerDetails[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("ae273202-b1c9-453e-afc5-b30af3bd217d")
                    IUserDataTaskDataProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskDataProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskDataProviderTriggerDetails = __uuidof(IUserDataTaskDataProviderTriggerDetails);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCompleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("f65e14a3-1a42-49da-8552-2873e52c55eb")
                    IUserDataTaskListCompleteTaskRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TaskListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            HSTRING completedTaskId,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListCompleteTaskRequest = __uuidof(IUserDataTaskListCompleteTaskRequest);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCompleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("d77c393d-4cf2-48ad-87fd-963f0eaa7a95")
                    IUserDataTaskListCompleteTaskRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCompleteTaskRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListCompleteTaskRequestEventArgs = __uuidof(IUserDataTaskListCompleteTaskRequestEventArgs);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCreateOrUpdateTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("2133772c-55c2-4300-8279-04326e07cce4")
                    IUserDataTaskListCreateOrUpdateTaskRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TaskListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Task(
                            ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask* createdOrUpdatedUserDataTask,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListCreateOrUpdateTaskRequest = __uuidof(IUserDataTaskListCreateOrUpdateTaskRequest);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCreateOrUpdateTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("12c55a52-e378-419b-ae38-a5e9e604476e")
                    IUserDataTaskListCreateOrUpdateTaskRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListCreateOrUpdateTaskRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListCreateOrUpdateTaskRequestEventArgs = __uuidof(IUserDataTaskListCreateOrUpdateTaskRequestEventArgs);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListDeleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("4b863c68-7657-4f3d-b074-d47ec8df07e7")
                    IUserDataTaskListDeleteTaskRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TaskListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListDeleteTaskRequest = __uuidof(IUserDataTaskListDeleteTaskRequest);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListDeleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("6063dad9-f562-4145-8efe-d50078c92b7f")
                    IUserDataTaskListDeleteTaskRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListDeleteTaskRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListDeleteTaskRequestEventArgs = __uuidof(IUserDataTaskListDeleteTaskRequestEventArgs);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSkipOccurrenceRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("ab87e34d-1cd3-431c-9f58-089aa4338d85")
                    IUserDataTaskListSkipOccurrenceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TaskListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListSkipOccurrenceRequest = __uuidof(IUserDataTaskListSkipOccurrenceRequest);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSkipOccurrenceRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("7a3b924a-cc2f-4e7b-aacd-a5b9d29cfa4e")
                    IUserDataTaskListSkipOccurrenceRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSkipOccurrenceRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListSkipOccurrenceRequestEventArgs = __uuidof(IUserDataTaskListSkipOccurrenceRequestEventArgs);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("40a73807-7590-4149-ae19-b211431a9f48")
                    IUserDataTaskListSyncManagerSyncRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TaskListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListSyncManagerSyncRequest = __uuidof(IUserDataTaskListSyncManagerSyncRequest);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                namespace DataProvider {
                    MIDL_INTERFACE("8ead1c12-768e-43bd-8385-5cdc351ffdea")
                    IUserDataTaskListSyncManagerSyncRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::UserDataTasks::DataProvider::IUserDataTaskListSyncManagerSyncRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataTaskListSyncManagerSyncRequestEventArgs = __uuidof(IUserDataTaskListSyncManagerSyncRequestEventArgs);
                } /* DataProvider */
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskDataProviderConnection[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CreateOrUpdateTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCreateOrUpdateTaskRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CreateOrUpdateTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSyncManagerSyncRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SkipOccurrenceRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListSkipOccurrenceRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SkipOccurrenceRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CompleteTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListCompleteTaskRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CompleteTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DeleteTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskDataProviderConnection_Windows__CApplicationModel__CUserDataTasks__CDataProvider__CUserDataTaskListDeleteTaskRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DeleteTaskRequested)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_add_CreateOrUpdateTaskRequested(This, handler, token) \
    ((This)->lpVtbl->add_CreateOrUpdateTaskRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_remove_CreateOrUpdateTaskRequested(This, token) \
    ((This)->lpVtbl->remove_CreateOrUpdateTaskRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_add_SyncRequested(This, handler, token) \
    ((This)->lpVtbl->add_SyncRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_remove_SyncRequested(This, token) \
    ((This)->lpVtbl->remove_SyncRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_add_SkipOccurrenceRequested(This, handler, token) \
    ((This)->lpVtbl->add_SkipOccurrenceRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_remove_SkipOccurrenceRequested(This, token) \
    ((This)->lpVtbl->remove_SkipOccurrenceRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_add_CompleteTaskRequested(This, handler, token) \
    ((This)->lpVtbl->add_CompleteTaskRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_remove_CompleteTaskRequested(This, token) \
    ((This)->lpVtbl->remove_CompleteTaskRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_add_DeleteTaskRequested(This, handler, token) \
    ((This)->lpVtbl->add_DeleteTaskRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_remove_DeleteTaskRequested(This, token) \
    ((This)->lpVtbl->remove_DeleteTaskRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskDataProviderTriggerDetails[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCompleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        HSTRING completedTaskId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_get_TaskListId(This, value) \
    ((This)->lpVtbl->get_TaskListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_ReportCompletedAsync(This, completedTaskId, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, completedTaskId, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCompleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCompleteTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCreateOrUpdateTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Task)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* createdOrUpdatedUserDataTask,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_get_TaskListId(This, value) \
    ((This)->lpVtbl->get_TaskListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_get_Task(This, value) \
    ((This)->lpVtbl->get_Task(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_ReportCompletedAsync(This, createdOrUpdatedUserDataTask, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, createdOrUpdatedUserDataTask, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListCreateOrUpdateTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListCreateOrUpdateTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListDeleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_get_TaskListId(This, value) \
    ((This)->lpVtbl->get_TaskListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListDeleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListDeleteTaskRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSkipOccurrenceRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_get_TaskListId(This, value) \
    ((This)->lpVtbl->get_TaskListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSkipOccurrenceRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSkipOccurrenceRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TaskListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_get_TaskListId(This, value) \
    ((This)->lpVtbl->get_TaskListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_DataProvider_IUserDataTaskListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CDataProvider_CIUserDataTaskListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderConnection[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskDataProviderTriggerDetails[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCompleteTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCompleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListCreateOrUpdateTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListCreateOrUpdateTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListDeleteTaskRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListDeleteTaskRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSkipOccurrenceRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSkipOccurrenceRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.DataProvider.IUserDataTaskListSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_DataProvider_UserDataTaskListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_p_h__

#endif // __windows2Eapplicationmodel2Euserdatatasks2Edataprovider_h__
