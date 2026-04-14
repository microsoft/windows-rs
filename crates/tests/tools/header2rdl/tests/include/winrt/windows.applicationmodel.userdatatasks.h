
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
#ifndef __windows2Eapplicationmodel2Euserdatatasks_h__
#define __windows2Eapplicationmodel2Euserdatatasks_h__
#ifndef __windows2Eapplicationmodel2Euserdatatasks_p_h__
#define __windows2Eapplicationmodel2Euserdatatasks_p_h__


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

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskBatch;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskBatch

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskListLimitedWriteOperations;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskListLimitedWriteOperations

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskListSyncManager;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskListSyncManager

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskManager;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskManager

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskManagerStatics;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskQueryOptions;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskQueryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskReader;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskReader

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskRecurrenceProperties;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRecurrenceProperties

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskRegenerationProperties;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRegenerationProperties

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                interface IUserDataTaskStore;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskStore

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__

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
            namespace UserDataTasks {
                class UserDataTask;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8b749b1-a847-5c34-866c-ea560cde1f49"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataTasks.UserDataTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a731c0cd-206d-5af8-ad64-85b7200f4ce2"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataTasks.UserDataTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskBatch;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d36b10df-1cab-544c-8c2e-3c7bc4d24b24"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e63bf8b8-91f0-5f13-a1c9-dee1879d0a52"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskBatch*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskBatch*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskList;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d9274ef2-63e0-544c-8db2-fe2d5b825d9e"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eefb814a-6af7-5d59-bf3f-b6c73b8c74a2"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskList*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskList>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskList*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskStore;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e02c4e60-78aa-5eb8-badb-ead22dca9679"))
IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33c9c615-2f80-587d-9f51-027ec457b7a3"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98733fc7-1908-532f-bfe3-3868445d29d6"))
IIterator<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.UserDataTasks.UserDataTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t;
#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8ae8becd-375f-5932-91ed-f82a74224365"))
IIterable<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.UserDataTasks.UserDataTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t;
#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0cfeaf9b-8758-5aa6-9806-32b0ea40d4b8"))
IVectorView<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataTasks.UserDataTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTask*> __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t;
#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskDaysOfWeek : unsigned int UserDataTaskDaysOfWeek;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_USE
#define DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("903861c5-d29d-5083-a03d-6b2a0233dcca"))
IReference<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskDaysOfWeek> : IReference_impl<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskDaysOfWeek>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskDaysOfWeek> __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_t;
#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek ABI::Windows::Foundation::__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskWeekOfMonth : int UserDataTaskWeekOfMonth;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_USE
#define DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5246fa7f-f9e9-5574-80f7-4aa359b0023d"))
IReference<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskWeekOfMonth> : IReference_impl<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskWeekOfMonth>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskWeekOfMonth> __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_t;
#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth ABI::Windows::Foundation::__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskListSyncManager;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("03e790f2-eacf-53eb-9060-6bfeec5bc375"))
ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListSyncManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListSyncManager*, ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskListSyncManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListSyncManager*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IPropertyValue;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIPropertyValue ABI::Windows::Foundation::IPropertyValue

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

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
            namespace UserDataTasks {
                typedef enum UserDataTaskDetailsKind : int UserDataTaskDetailsKind;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskKind : int UserDataTaskKind;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskListOtherAppReadAccess : int UserDataTaskListOtherAppReadAccess;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskListOtherAppWriteAccess : int UserDataTaskListOtherAppWriteAccess;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskListSyncStatus : int UserDataTaskListSyncStatus;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskPriority : int UserDataTaskPriority;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskQueryKind : int UserDataTaskQueryKind;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskQuerySortProperty : int UserDataTaskQuerySortProperty;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskRecurrenceUnit : int UserDataTaskRecurrenceUnit;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskRegenerationUnit : int UserDataTaskRegenerationUnit;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskSensitivity : int UserDataTaskSensitivity;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                typedef enum UserDataTaskStoreAccessType : int UserDataTaskStoreAccessType;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskListLimitedWriteOperations;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskManager;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskQueryOptions;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskReader;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskRecurrenceProperties;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                class UserDataTaskRegenerationProperties;
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskDaysOfWeek : unsigned int
                {
                    UserDataTaskDaysOfWeek_None = 0,
                    UserDataTaskDaysOfWeek_Sunday = 0x1,
                    UserDataTaskDaysOfWeek_Monday = 0x2,
                    UserDataTaskDaysOfWeek_Tuesday = 0x4,
                    UserDataTaskDaysOfWeek_Wednesday = 0x8,
                    UserDataTaskDaysOfWeek_Thursday = 0x10,
                    UserDataTaskDaysOfWeek_Friday = 0x20,
                    UserDataTaskDaysOfWeek_Saturday = 0x40,
                };

                DEFINE_ENUM_FLAG_OPERATORS(UserDataTaskDaysOfWeek)
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskDetailsKind : int
                {
                    UserDataTaskDetailsKind_PlainText = 0,
                    UserDataTaskDetailsKind_Html = 1,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskKind : int
                {
                    UserDataTaskKind_Single = 0,
                    UserDataTaskKind_Recurring = 1,
                    UserDataTaskKind_Regenerating = 2,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskListOtherAppReadAccess : int
                {
                    UserDataTaskListOtherAppReadAccess_Full = 0,
                    UserDataTaskListOtherAppReadAccess_SystemOnly = 1,
                    UserDataTaskListOtherAppReadAccess_None = 2,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskListOtherAppWriteAccess : int
                {
                    UserDataTaskListOtherAppWriteAccess_Limited = 0,
                    UserDataTaskListOtherAppWriteAccess_None = 1,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskListSyncStatus : int
                {
                    UserDataTaskListSyncStatus_Idle = 0,
                    UserDataTaskListSyncStatus_Syncing = 1,
                    UserDataTaskListSyncStatus_UpToDate = 2,
                    UserDataTaskListSyncStatus_AuthenticationError = 3,
                    UserDataTaskListSyncStatus_PolicyError = 4,
                    UserDataTaskListSyncStatus_UnknownError = 5,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskPriority : int
                {
                    UserDataTaskPriority_Normal = 0,
                    UserDataTaskPriority_Low = -1,
                    UserDataTaskPriority_High = 1,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskQueryKind : int
                {
                    UserDataTaskQueryKind_All = 0,
                    UserDataTaskQueryKind_Incomplete = 1,
                    UserDataTaskQueryKind_Complete = 2,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskQuerySortProperty : int
                {
                    UserDataTaskQuerySortProperty_DueDate = 0,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskRecurrenceUnit : int
                {
                    UserDataTaskRecurrenceUnit_Daily = 0,
                    UserDataTaskRecurrenceUnit_Weekly = 1,
                    UserDataTaskRecurrenceUnit_Monthly = 2,
                    UserDataTaskRecurrenceUnit_MonthlyOnDay = 3,
                    UserDataTaskRecurrenceUnit_Yearly = 4,
                    UserDataTaskRecurrenceUnit_YearlyOnDay = 5,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskRegenerationUnit : int
                {
                    UserDataTaskRegenerationUnit_Daily = 0,
                    UserDataTaskRegenerationUnit_Weekly = 1,
                    UserDataTaskRegenerationUnit_Monthly = 2,
                    UserDataTaskRegenerationUnit_Yearly = 4,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskSensitivity : int
                {
                    UserDataTaskSensitivity_Public = 0,
                    UserDataTaskSensitivity_Private = 1,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskStoreAccessType : int
                {
                    UserDataTaskStoreAccessType_AppTasksReadWrite = 0,
                    UserDataTaskStoreAccessType_AllTasksLimitedReadWrite = 1,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                enum UserDataTaskWeekOfMonth : int
                {
                    UserDataTaskWeekOfMonth_First = 0,
                    UserDataTaskWeekOfMonth_Second = 1,
                    UserDataTaskWeekOfMonth_Third = 2,
                    UserDataTaskWeekOfMonth_Fourth = 3,
                    UserDataTaskWeekOfMonth_Last = 4,
                };
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTask[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTask";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("7c6585d1-e0d4-4f99-aee2-bc2d5ddadf4c")
                IUserDataTask : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ListId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CompletedDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CompletedDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Details(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetailsKind(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskDetailsKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DetailsKind(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskDetailsKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DueDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DueDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Priority(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskPriority* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Priority(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskPriority value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecurrenceProperties(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRecurrenceProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RecurrenceProperties(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRecurrenceProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RegenerationProperties(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRegenerationProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RegenerationProperties(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskRegenerationProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reminder(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Reminder(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sensitivity(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskSensitivity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Sensitivity(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskSensitivity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subject(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subject(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTask = __uuidof(IUserDataTask);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskBatch[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("382da5fe-20b5-431c-8f42-a5d292ec930c")
                IUserDataTaskBatch : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Tasks(
                        __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskBatch = __uuidof(IUserDataTaskBatch);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskList[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskList";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("49412e39-7c1d-4df1-bed3-314b7cbf5e4e")
                IUserDataTaskList : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserDataAccountId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListOtherAppReadAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListOtherAppReadAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppWriteAccess(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListOtherAppWriteAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppWriteAccess(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListOtherAppWriteAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LimitedWriteOperations(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskListLimitedWriteOperations** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SyncManager(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskListSyncManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RegisterSyncManagerAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTaskReader(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTaskReaderWithOptions(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskQueryOptions* options,
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTaskAsync(
                        HSTRING userDataTask,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveTaskAsync(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask* userDataTask,
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteTaskAsync(
                        HSTRING userDataTaskId,
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskList = __uuidof(IUserDataTaskList);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskListLimitedWriteOperations[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("7aa267f2-6078-4183-919e-4f29f19cfae9")
                IUserDataTaskListLimitedWriteOperations : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCompleteTaskAsync(
                        HSTRING userDataTaskId,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateOrUpdateTaskAsync(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTask* userDataTask,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryDeleteTaskAsync(
                        HSTRING userDataTaskId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySkipOccurrenceAsync(
                        HSTRING userDataTaskId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskListLimitedWriteOperations = __uuidof(IUserDataTaskListLimitedWriteOperations);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskListSyncManager[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("8e591a95-1dcf-469f-93ec-ba48bb553c6b")
                IUserDataTaskListSyncManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LastAttemptedSyncTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastAttemptedSyncTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastSuccessfulSyncTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastSuccessfulSyncTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListSyncStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Status(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskListSyncStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SyncAsync(
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SyncStatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SyncStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskListSyncManager = __uuidof(IUserDataTaskListSyncManager);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskManager[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("8451c914-e60b-48a9-9211-7fb8a56cb84c")
                IUserDataTaskManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskStoreAccessType accessType,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskManager = __uuidof(IUserDataTaskManager);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskManagerStatics[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("b35539f8-c502-47fc-a81e-100883719d55")
                IUserDataTaskManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::ApplicationModel::UserDataTasks::IUserDataTaskManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskManagerStatics = __uuidof(IUserDataTaskManagerStatics);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskQueryOptions[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("959f27ed-909a-4d30-8c1b-331d8fe667e2")
                IUserDataTaskQueryOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SortProperty(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskQuerySortProperty* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SortProperty(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskQuerySortProperty value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskQueryKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Kind(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskQueryKind value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskQueryOptions = __uuidof(IUserDataTaskQueryOptions);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskReader[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("03e688b1-4ccf-4500-883b-e76290cfed63")
                IUserDataTaskReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskReader = __uuidof(IUserDataTaskReader);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskRecurrenceProperties[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("73df80b0-27c6-40ce-b149-9cd41485a69e")
                IUserDataTaskRecurrenceProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Unit(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskRecurrenceUnit* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Unit(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskRecurrenceUnit value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Occurrences(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Occurrences(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Interval(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Interval(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DaysOfWeek(
                        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DaysOfWeek(
                        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WeekOfMonth(
                        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WeekOfMonth(
                        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Month(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Month(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Day(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Day(
                        __FIReference_1_int* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskRecurrenceProperties = __uuidof(IUserDataTaskRecurrenceProperties);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskRegenerationProperties[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("92ab0007-090e-4704-bb5c-84fc0b0d9c31")
                IUserDataTaskRegenerationProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Unit(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskRegenerationUnit* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Unit(
                        ABI::Windows::ApplicationModel::UserDataTasks::UserDataTaskRegenerationUnit value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Occurrences(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Occurrences(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Interval(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Interval(
                        INT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskRegenerationProperties = __uuidof(IUserDataTaskRegenerationProperties);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskStore[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataTasks {
                MIDL_INTERFACE("f06a9cb0-f1db-45ba-8a62-086004c0213d")
                IUserDataTaskStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateListAsync(
                        HSTRING name,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateListInAccountAsync(
                        HSTRING name,
                        HSTRING userDataAccountId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindListsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetListAsync(
                        HSTRING taskListId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserDataTaskStore = __uuidof(IUserDataTaskStore);
            } /* UserDataTasks */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTask_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTask[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskList ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskList_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskList[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

typedef struct __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

typedef struct __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __FIIterator_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDaysOfWeek __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDaysOfWeek;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek;

typedef struct __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeekVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDaysOfWeek* result);

    END_INTERFACE
} __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeekVtbl;

interface __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek
{
    CONST_VTBL struct __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeekVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskWeekOfMonth __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskWeekOfMonth;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth;

typedef struct __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonthVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskWeekOfMonth* result);

    END_INTERFACE
} __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonthVtbl;

interface __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth
{
    CONST_VTBL struct __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonthVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDetailsKind __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDetailsKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskKind __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppReadAccess __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppReadAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppWriteAccess __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppWriteAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListSyncStatus __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListSyncStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskPriority __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskPriority;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQueryKind __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQueryKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQuerySortProperty __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQuerySortProperty;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRecurrenceUnit __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRecurrenceUnit;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRegenerationUnit __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRegenerationUnit;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskSensitivity __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskSensitivity;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskStoreAccessType __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskStoreAccessType;

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDaysOfWeek
{
    UserDataTaskDaysOfWeek_None = 0,
    UserDataTaskDaysOfWeek_Sunday = 0x1,
    UserDataTaskDaysOfWeek_Monday = 0x2,
    UserDataTaskDaysOfWeek_Tuesday = 0x4,
    UserDataTaskDaysOfWeek_Wednesday = 0x8,
    UserDataTaskDaysOfWeek_Thursday = 0x10,
    UserDataTaskDaysOfWeek_Friday = 0x20,
    UserDataTaskDaysOfWeek_Saturday = 0x40,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDetailsKind
{
    UserDataTaskDetailsKind_PlainText = 0,
    UserDataTaskDetailsKind_Html = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskKind
{
    UserDataTaskKind_Single = 0,
    UserDataTaskKind_Recurring = 1,
    UserDataTaskKind_Regenerating = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppReadAccess
{
    UserDataTaskListOtherAppReadAccess_Full = 0,
    UserDataTaskListOtherAppReadAccess_SystemOnly = 1,
    UserDataTaskListOtherAppReadAccess_None = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppWriteAccess
{
    UserDataTaskListOtherAppWriteAccess_Limited = 0,
    UserDataTaskListOtherAppWriteAccess_None = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListSyncStatus
{
    UserDataTaskListSyncStatus_Idle = 0,
    UserDataTaskListSyncStatus_Syncing = 1,
    UserDataTaskListSyncStatus_UpToDate = 2,
    UserDataTaskListSyncStatus_AuthenticationError = 3,
    UserDataTaskListSyncStatus_PolicyError = 4,
    UserDataTaskListSyncStatus_UnknownError = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskPriority
{
    UserDataTaskPriority_Normal = 0,
    UserDataTaskPriority_Low = -1,
    UserDataTaskPriority_High = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQueryKind
{
    UserDataTaskQueryKind_All = 0,
    UserDataTaskQueryKind_Incomplete = 1,
    UserDataTaskQueryKind_Complete = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQuerySortProperty
{
    UserDataTaskQuerySortProperty_DueDate = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRecurrenceUnit
{
    UserDataTaskRecurrenceUnit_Daily = 0,
    UserDataTaskRecurrenceUnit_Weekly = 1,
    UserDataTaskRecurrenceUnit_Monthly = 2,
    UserDataTaskRecurrenceUnit_MonthlyOnDay = 3,
    UserDataTaskRecurrenceUnit_Yearly = 4,
    UserDataTaskRecurrenceUnit_YearlyOnDay = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRegenerationUnit
{
    UserDataTaskRegenerationUnit_Daily = 0,
    UserDataTaskRegenerationUnit_Weekly = 1,
    UserDataTaskRegenerationUnit_Monthly = 2,
    UserDataTaskRegenerationUnit_Yearly = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskSensitivity
{
    UserDataTaskSensitivity_Public = 0,
    UserDataTaskSensitivity_Private = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskStoreAccessType
{
    UserDataTaskStoreAccessType_AppTasksReadWrite = 0,
    UserDataTaskStoreAccessType_AllTasksLimitedReadWrite = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskWeekOfMonth
{
    UserDataTaskWeekOfMonth_First = 0,
    UserDataTaskWeekOfMonth_Second = 1,
    UserDataTaskWeekOfMonth_Third = 2,
    UserDataTaskWeekOfMonth_Fourth = 3,
    UserDataTaskWeekOfMonth_Last = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTask
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTask[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTask";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ListId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CompletedDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_CompletedDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Details)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DetailsKind)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDetailsKind* value);
    HRESULT (STDMETHODCALLTYPE* put_DetailsKind)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskDetailsKind value);
    HRESULT (STDMETHODCALLTYPE* get_DueDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_DueDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskPriority* value);
    HRESULT (STDMETHODCALLTYPE* put_Priority)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskPriority value);
    HRESULT (STDMETHODCALLTYPE* get_RecurrenceProperties)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties** value);
    HRESULT (STDMETHODCALLTYPE* put_RecurrenceProperties)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_RegenerationProperties)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties** value);
    HRESULT (STDMETHODCALLTYPE* put_RegenerationProperties)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_Reminder)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Reminder)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Sensitivity)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskSensitivity* value);
    HRESULT (STDMETHODCALLTYPE* put_Sensitivity)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskSensitivity value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subject)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_StartDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_StartDate)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_ListId(This, value) \
    ((This)->lpVtbl->get_ListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_CompletedDate(This, value) \
    ((This)->lpVtbl->get_CompletedDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_CompletedDate(This, value) \
    ((This)->lpVtbl->put_CompletedDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_Details(This, value) \
    ((This)->lpVtbl->put_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_DetailsKind(This, value) \
    ((This)->lpVtbl->get_DetailsKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_DetailsKind(This, value) \
    ((This)->lpVtbl->put_DetailsKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_DueDate(This, value) \
    ((This)->lpVtbl->get_DueDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_DueDate(This, value) \
    ((This)->lpVtbl->put_DueDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_Priority(This, value) \
    ((This)->lpVtbl->put_Priority(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_RecurrenceProperties(This, value) \
    ((This)->lpVtbl->get_RecurrenceProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_RecurrenceProperties(This, value) \
    ((This)->lpVtbl->put_RecurrenceProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_RegenerationProperties(This, value) \
    ((This)->lpVtbl->get_RegenerationProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_RegenerationProperties(This, value) \
    ((This)->lpVtbl->put_RegenerationProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Reminder(This, value) \
    ((This)->lpVtbl->get_Reminder(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_Reminder(This, value) \
    ((This)->lpVtbl->put_Reminder(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Sensitivity(This, value) \
    ((This)->lpVtbl->get_Sensitivity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_Sensitivity(This, value) \
    ((This)->lpVtbl->put_Sensitivity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_Subject(This, value) \
    ((This)->lpVtbl->put_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_get_StartDate(This, value) \
    ((This)->lpVtbl->get_StartDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_put_StartDate(This, value) \
    ((This)->lpVtbl->put_StartDate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskBatch[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatchVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Tasks)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch* This,
        __FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatchVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatchVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_get_Tasks(This, value) \
    ((This)->lpVtbl->get_Tasks(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskBatch_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskList
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskList[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskList";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserDataAccountId)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SourceDisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppReadAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppReadAccess value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppWriteAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppWriteAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppWriteAccess)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListOtherAppWriteAccess value);
    HRESULT (STDMETHODCALLTYPE* get_LimitedWriteOperations)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations** value);
    HRESULT (STDMETHODCALLTYPE* get_SyncManager)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager** value);
    HRESULT (STDMETHODCALLTYPE* RegisterSyncManagerAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetTaskReader)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader** result);
    HRESULT (STDMETHODCALLTYPE* GetTaskReaderWithOptions)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* options,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader** value);
    HRESULT (STDMETHODCALLTYPE* GetTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING userDataTask,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTask** operation);
    HRESULT (STDMETHODCALLTYPE* SaveTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* userDataTask,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* DeleteTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        HSTRING userDataTaskId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_UserDataAccountId(This, value) \
    ((This)->lpVtbl->get_UserDataAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_SourceDisplayName(This, value) \
    ((This)->lpVtbl->get_SourceDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_put_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_OtherAppWriteAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppWriteAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_put_OtherAppWriteAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppWriteAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_LimitedWriteOperations(This, value) \
    ((This)->lpVtbl->get_LimitedWriteOperations(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_get_SyncManager(This, value) \
    ((This)->lpVtbl->get_SyncManager(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_RegisterSyncManagerAsync(This, result) \
    ((This)->lpVtbl->RegisterSyncManagerAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetTaskReader(This, result) \
    ((This)->lpVtbl->GetTaskReader(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetTaskReaderWithOptions(This, options, value) \
    ((This)->lpVtbl->GetTaskReaderWithOptions(This, options, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_GetTaskAsync(This, userDataTask, operation) \
    ((This)->lpVtbl->GetTaskAsync(This, userDataTask, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_SaveTaskAsync(This, userDataTask, action) \
    ((This)->lpVtbl->SaveTaskAsync(This, userDataTask, action))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_DeleteTaskAsync(This, userDataTaskId, action) \
    ((This)->lpVtbl->DeleteTaskAsync(This, userDataTaskId, action))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_DeleteAsync(This, asyncAction) \
    ((This)->lpVtbl->DeleteAsync(This, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_SaveAsync(This, asyncAction) \
    ((This)->lpVtbl->SaveAsync(This, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskListLimitedWriteOperations[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperationsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCompleteTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        HSTRING userDataTaskId,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* TryCreateOrUpdateTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTask* userDataTask,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryDeleteTaskAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        HSTRING userDataTaskId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TrySkipOccurrenceAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations* This,
        HSTRING userDataTaskId,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperationsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperationsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_TryCompleteTaskAsync(This, userDataTaskId, operation) \
    ((This)->lpVtbl->TryCompleteTaskAsync(This, userDataTaskId, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_TryCreateOrUpdateTaskAsync(This, userDataTask, operation) \
    ((This)->lpVtbl->TryCreateOrUpdateTaskAsync(This, userDataTask, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_TryDeleteTaskAsync(This, userDataTaskId, operation) \
    ((This)->lpVtbl->TryDeleteTaskAsync(This, userDataTaskId, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_TrySkipOccurrenceAsync(This, userDataTaskId, operation) \
    ((This)->lpVtbl->TrySkipOccurrenceAsync(This, userDataTaskId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListLimitedWriteOperations_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskListSyncManager[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LastAttemptedSyncTime)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_LastAttemptedSyncTime)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_LastSuccessfulSyncTime)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_LastSuccessfulSyncTime)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListSyncStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskListSyncStatus value);
    HRESULT (STDMETHODCALLTYPE* SyncAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* add_SyncStatusChanged)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskListSyncManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SyncStatusChanged)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_get_LastAttemptedSyncTime(This, value) \
    ((This)->lpVtbl->get_LastAttemptedSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_put_LastAttemptedSyncTime(This, value) \
    ((This)->lpVtbl->put_LastAttemptedSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_get_LastSuccessfulSyncTime(This, value) \
    ((This)->lpVtbl->get_LastSuccessfulSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_put_LastSuccessfulSyncTime(This, value) \
    ((This)->lpVtbl->put_LastSuccessfulSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_SyncAsync(This, result) \
    ((This)->lpVtbl->SyncAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_add_SyncStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_SyncStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_remove_SyncStatusChanged(This, token) \
    ((This)->lpVtbl->remove_SyncStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskListSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskManager[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskStoreAccessType accessType,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskStore** operation);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_RequestStoreAsync(This, accessType, operation) \
    ((This)->lpVtbl->RequestStoreAsync(This, accessType, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskManagerStatics[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager** result);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskQueryOptions[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SortProperty)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQuerySortProperty* value);
    HRESULT (STDMETHODCALLTYPE* put_SortProperty)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQuerySortProperty value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQueryKind* value);
    HRESULT (STDMETHODCALLTYPE* put_Kind)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskQueryKind value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_get_SortProperty(This, value) \
    ((This)->lpVtbl->get_SortProperty(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_put_SortProperty(This, value) \
    ((This)->lpVtbl->put_SortProperty(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_put_Kind(This, value) \
    ((This)->lpVtbl->put_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskReader[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskBatch** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_ReadBatchAsync(This, result) \
    ((This)->lpVtbl->ReadBatchAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskRecurrenceProperties[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrencePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unit)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRecurrenceUnit* value);
    HRESULT (STDMETHODCALLTYPE* put_Unit)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRecurrenceUnit value);
    HRESULT (STDMETHODCALLTYPE* get_Occurrences)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_Occurrences)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_Until)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Until)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Interval)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Interval)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DaysOfWeek)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek** value);
    HRESULT (STDMETHODCALLTYPE* put_DaysOfWeek)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskDaysOfWeek* value);
    HRESULT (STDMETHODCALLTYPE* get_WeekOfMonth)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth** value);
    HRESULT (STDMETHODCALLTYPE* put_WeekOfMonth)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskWeekOfMonth* value);
    HRESULT (STDMETHODCALLTYPE* get_Month)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_Month)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_Day)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_Day)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties* This,
        __FIReference_1_int* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrencePropertiesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrencePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Unit(This, value) \
    ((This)->lpVtbl->get_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Unit(This, value) \
    ((This)->lpVtbl->put_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Occurrences(This, value) \
    ((This)->lpVtbl->get_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Occurrences(This, value) \
    ((This)->lpVtbl->put_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Until(This, value) \
    ((This)->lpVtbl->get_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Until(This, value) \
    ((This)->lpVtbl->put_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Interval(This, value) \
    ((This)->lpVtbl->get_Interval(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Interval(This, value) \
    ((This)->lpVtbl->put_Interval(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_DaysOfWeek(This, value) \
    ((This)->lpVtbl->get_DaysOfWeek(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_DaysOfWeek(This, value) \
    ((This)->lpVtbl->put_DaysOfWeek(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_WeekOfMonth(This, value) \
    ((This)->lpVtbl->get_WeekOfMonth(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_WeekOfMonth(This, value) \
    ((This)->lpVtbl->put_WeekOfMonth(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Month(This, value) \
    ((This)->lpVtbl->get_Month(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Month(This, value) \
    ((This)->lpVtbl->put_Month(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_get_Day(This, value) \
    ((This)->lpVtbl->get_Day(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_put_Day(This, value) \
    ((This)->lpVtbl->put_Day(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRecurrenceProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskRegenerationProperties[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unit)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRegenerationUnit* value);
    HRESULT (STDMETHODCALLTYPE* put_Unit)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CUserDataTaskRegenerationUnit value);
    HRESULT (STDMETHODCALLTYPE* get_Occurrences)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_Occurrences)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_Until)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Until)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Interval)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Interval)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationPropertiesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_get_Unit(This, value) \
    ((This)->lpVtbl->get_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_put_Unit(This, value) \
    ((This)->lpVtbl->put_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_get_Occurrences(This, value) \
    ((This)->lpVtbl->get_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_put_Occurrences(This, value) \
    ((This)->lpVtbl->put_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_get_Until(This, value) \
    ((This)->lpVtbl->get_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_put_Until(This, value) \
    ((This)->lpVtbl->put_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_get_Interval(This, value) \
    ((This)->lpVtbl->get_Interval(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_put_Interval(This, value) \
    ((This)->lpVtbl->put_Interval(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskRegenerationProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataTasks.UserDataTaskStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataTasks_IUserDataTaskStore[] = L"Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateListAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation);
    HRESULT (STDMETHODCALLTYPE* CreateListInAccountAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        HSTRING name,
        HSTRING userDataAccountId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** result);
    HRESULT (STDMETHODCALLTYPE* FindListsAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation);
    HRESULT (STDMETHODCALLTYPE* GetListAsync)(__x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore* This,
        HSTRING taskListId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CUserDataTasks__CUserDataTaskList** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_CreateListAsync(This, name, operation) \
    ((This)->lpVtbl->CreateListAsync(This, name, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_CreateListInAccountAsync(This, name, userDataAccountId, result) \
    ((This)->lpVtbl->CreateListInAccountAsync(This, name, userDataAccountId, result))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_FindListsAsync(This, operation) \
    ((This)->lpVtbl->FindListsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_GetListAsync(This, taskListId, operation) \
    ((This)->lpVtbl->GetListAsync(This, taskListId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataTasks_CIUserDataTaskStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTask
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTask_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTask[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTask";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskBatch[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskList ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskList_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskList[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskListLimitedWriteOperations[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskListSyncManager[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskManager[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskQueryOptions[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskReader[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskRecurrenceProperties[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskRegenerationProperties[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.UserDataTasks.UserDataTaskStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataTasks_UserDataTaskStore[] = L"Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
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
#endif // __windows2Eapplicationmodel2Euserdatatasks_p_h__

#endif // __windows2Eapplicationmodel2Euserdatatasks_h__
