
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
#ifndef __windows2Eapplicationmodel2Eappservice_h__
#define __windows2Eapplicationmodel2Eappservice_h__
#ifndef __windows2Eapplicationmodel2Eappservice_p_h__
#define __windows2Eapplicationmodel2Eappservice_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.h"
#include "Windows.System.h"
#include "Windows.System.RemoteSystems.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceCatalogStatics;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics ABI::Windows::ApplicationModel::AppService::IAppServiceCatalogStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceClosedEventArgs;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs ABI::Windows::ApplicationModel::AppService::IAppServiceClosedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection ABI::Windows::ApplicationModel::AppService::IAppServiceConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceConnection2;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2 ABI::Windows::ApplicationModel::AppService::IAppServiceConnection2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceConnectionStatics;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics ABI::Windows::ApplicationModel::AppService::IAppServiceConnectionStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceDeferral;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral ABI::Windows::ApplicationModel::AppService::IAppServiceDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceRequest;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest ABI::Windows::ApplicationModel::AppService::IAppServiceRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceRequestReceivedEventArgs;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs ABI::Windows::ApplicationModel::AppService::IAppServiceRequestReceivedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceResponse;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse ABI::Windows::ApplicationModel::AppService::IAppServiceResponse

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceTriggerDetails;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails ABI::Windows::ApplicationModel::AppService::IAppServiceTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceTriggerDetails2;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2 ABI::Windows::ApplicationModel::AppService::IAppServiceTriggerDetails2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceTriggerDetails3;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3 ABI::Windows::ApplicationModel::AppService::IAppServiceTriggerDetails3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceTriggerDetails4;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4 ABI::Windows::ApplicationModel::AppService::IAppServiceTriggerDetails4

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IStatelessAppServiceResponse;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse ABI::Windows::ApplicationModel::AppService::IStatelessAppServiceResponse

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace AppService {
                typedef enum AppServiceConnectionStatus : int AppServiceConnectionStatus;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d0e6663-2639-5a9a-9cbc-30d7d4ce533b"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.AppService.AppServiceConnectionStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus> __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b6c6bbf2-72ca-5799-a651-d1990670097b"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.AppService.AppServiceConnectionStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::AppService::AppServiceConnectionStatus> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceResponse;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("48755a7c-c88f-5ef0-9b4c-876fcc2610b4"))
IAsyncOperation<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*, ABI::Windows::ApplicationModel::AppService::IAppServiceResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.AppService.AppServiceResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7ea7d7ec-e164-52c3-8e32-bba7126d9028"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*, ABI::Windows::ApplicationModel::AppService::IAppServiceResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.AppService.AppServiceResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::AppService::AppServiceResponse*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                typedef enum AppServiceResponseStatus : int AppServiceResponseStatus;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("98fdb842-5a0b-539a-a35c-55ac5f228612"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.AppService.AppServiceResponseStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus> __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b824383d-32e0-5579-8670-a06a61457f20"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.AppService.AppServiceResponseStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class StatelessAppServiceResponse;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b2a1524e-beab-5beb-aae9-b43501c3a488"))
IAsyncOperation<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*, ABI::Windows::ApplicationModel::AppService::IStatelessAppServiceResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.AppService.StatelessAppServiceResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4d1ee4ed-4d79-53fa-a9d7-c9c9354e4f55"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*, ABI::Windows::ApplicationModel::AppService::IStatelessAppServiceResponse*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.AppService.StatelessAppServiceResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponse*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo ABI::Windows::ApplicationModel::IAppInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppInfo_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("69cec62c-41eb-5d69-a475-29ee22323dd8"))
IIterator<ABI::Windows::ApplicationModel::AppInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInfo*, ABI::Windows::ApplicationModel::IAppInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.AppInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::AppInfo*> __FIIterator_1_Windows__CApplicationModel__CAppInfo_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppInfo_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63d0bffe-0e34-55b3-83d5-314caff2b137"))
IIterable<ABI::Windows::ApplicationModel::AppInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInfo*, ABI::Windows::ApplicationModel::IAppInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.AppInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::AppInfo*> __FIIterable_1_Windows__CApplicationModel__CAppInfo_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8246ed12-33e8-52b3-a5c5-19779de9999e"))
IVectorView<ABI::Windows::ApplicationModel::AppInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInfo*, ABI::Windows::ApplicationModel::IAppInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::AppInfo*> __FIVectorView_1_Windows__CApplicationModel__CAppInfo_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07543d91-8610-5152-b0e4-43d6e4cdd0cb"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07f25b6f-f054-5649-a5ce-b348ddc618b6"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppInfo>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppInfo*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceClosedEventArgs;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e4efa98d-4bfc-5e61-a233-688f5f06521f"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::AppServiceClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::IAppServiceConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceClosedEventArgs*, ABI::Windows::ApplicationModel::AppService::IAppServiceClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppService.AppServiceConnection, Windows.ApplicationModel.AppService.AppServiceClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::AppServiceClosedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceRequestReceivedEventArgs;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("18c67d61-4176-5553-b18d-d8f57fe79552"))
ITypedEventHandler<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::AppServiceRequestReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::IAppServiceConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppService::AppServiceRequestReceivedEventArgs*, ABI::Windows::ApplicationModel::AppService::IAppServiceRequestReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.AppService.AppServiceConnection, Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::AppService::AppServiceConnection*, ABI::Windows::ApplicationModel::AppService::AppServiceRequestReceivedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                class RemoteSystemConnectionRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteSystems {
                interface IRemoteSystemConnectionRequest;
            } /* RemoteSystems */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__

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
            namespace AppService {
                typedef enum AppServiceClosedStatus : int AppServiceClosedStatus;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                typedef enum StatelessAppServiceResponseStatus : int StatelessAppServiceResponseStatus;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceDeferral;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceRequest;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceClosedStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                enum AppServiceClosedStatus : int
                {
                    AppServiceClosedStatus_Completed = 0,
                    AppServiceClosedStatus_Canceled = 1,
                    AppServiceClosedStatus_ResourceLimitsExceeded = 2,
                    AppServiceClosedStatus_Unknown = 3,
                };
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                enum AppServiceConnectionStatus : int
                {
                    AppServiceConnectionStatus_Success = 0,
                    AppServiceConnectionStatus_AppNotInstalled = 1,
                    AppServiceConnectionStatus_AppUnavailable = 2,
                    AppServiceConnectionStatus_AppServiceUnavailable = 3,
                    AppServiceConnectionStatus_Unknown = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppServiceConnectionStatus_RemoteSystemUnavailable = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppServiceConnectionStatus_RemoteSystemNotSupportedByApp = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppServiceConnectionStatus_NotAuthorized = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceConnectionStatus_AuthenticationError = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceConnectionStatus_NetworkNotAvailable = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceConnectionStatus_DisabledByPolicy = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceConnectionStatus_WebServiceUnavailable = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                };
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceResponseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                enum AppServiceResponseStatus : int
                {
                    AppServiceResponseStatus_Success = 0,
                    AppServiceResponseStatus_Failure = 1,
                    AppServiceResponseStatus_ResourceLimitsExceeded = 2,
                    AppServiceResponseStatus_Unknown = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppServiceResponseStatus_RemoteSystemUnavailable = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppServiceResponseStatus_MessageSizeTooLarge = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceResponseStatus_AppUnavailable = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceResponseStatus_AuthenticationError = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceResponseStatus_NetworkNotAvailable = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceResponseStatus_DisabledByPolicy = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    AppServiceResponseStatus_WebServiceUnavailable = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                };
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                enum StatelessAppServiceResponseStatus : int
                {
                    StatelessAppServiceResponseStatus_Success = 0,
                    StatelessAppServiceResponseStatus_AppNotInstalled = 1,
                    StatelessAppServiceResponseStatus_AppUnavailable = 2,
                    StatelessAppServiceResponseStatus_AppServiceUnavailable = 3,
                    StatelessAppServiceResponseStatus_RemoteSystemUnavailable = 4,
                    StatelessAppServiceResponseStatus_RemoteSystemNotSupportedByApp = 5,
                    StatelessAppServiceResponseStatus_NotAuthorized = 6,
                    StatelessAppServiceResponseStatus_ResourceLimitsExceeded = 7,
                    StatelessAppServiceResponseStatus_MessageSizeTooLarge = 8,
                    StatelessAppServiceResponseStatus_Failure = 9,
                    StatelessAppServiceResponseStatus_Unknown = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    StatelessAppServiceResponseStatus_AuthenticationError = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    StatelessAppServiceResponseStatus_NetworkNotAvailable = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    StatelessAppServiceResponseStatus_DisabledByPolicy = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    StatelessAppServiceResponseStatus_WebServiceUnavailable = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                };
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceCatalogStatics[] = L"Windows.ApplicationModel.AppService.IAppServiceCatalogStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("ef0d2507-d132-4c85-8395-3c31d5a1e941")
                IAppServiceCatalogStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAppServiceProvidersAsync(
                        HSTRING appServiceName,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceCatalogStatics = __uuidof(IAppServiceCatalogStatics);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceClosedEventArgs[] = L"Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("de6016f6-cb03-4d35-ac8d-cc6303239731")
                IAppServiceClosedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::AppService::AppServiceClosedStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceClosedEventArgs = __uuidof(IAppServiceClosedEventArgs);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnection[] = L"Windows.ApplicationModel.AppService.IAppServiceConnection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("9dd474a2-871f-4d52-89a9-9e090531bd27")
                IAppServiceConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AppServiceName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PackageFamilyName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendMessageAsync(
                        ABI::Windows::Foundation::Collections::IPropertySet* message,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RequestReceived(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RequestReceived(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ServiceClosed(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ServiceClosed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceConnection = __uuidof(IAppServiceConnection);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnection2[] = L"Windows.ApplicationModel.AppService.IAppServiceConnection2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("8bdfcd5f-2302-4fbd-8061-52511c2f8bf9")
                IAppServiceConnection2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE OpenRemoteAsync(
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest* remoteSystemConnectionRequest,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_User(
                        ABI::Windows::System::IUser* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceConnection2 = __uuidof(IAppServiceConnection2);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnectionStatics[] = L"Windows.ApplicationModel.AppService.IAppServiceConnectionStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("adc56ce9-d408-5673-8637-827a4b274168")
                IAppServiceConnectionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SendStatelessMessageAsync(
                        ABI::Windows::ApplicationModel::AppService::IAppServiceConnection* connection,
                        ABI::Windows::System::RemoteSystems::IRemoteSystemConnectionRequest* connectionRequest,
                        ABI::Windows::Foundation::Collections::IPropertySet* message,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceConnectionStatics = __uuidof(IAppServiceConnectionStatics);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceDeferral[] = L"Windows.ApplicationModel.AppService.IAppServiceDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("7e1b5322-eab0-4248-ae04-fdf93838e472")
                IAppServiceDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceDeferral = __uuidof(IAppServiceDeferral);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceRequest[] = L"Windows.ApplicationModel.AppService.IAppServiceRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("20e58d9d-18de-4b01-80ba-90a76204e3c8")
                IAppServiceRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendResponseAsync(
                        ABI::Windows::Foundation::Collections::IPropertySet* message,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceRequest = __uuidof(IAppServiceRequest);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceRequestReceivedEventArgs[] = L"Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("6e122360-ff65-44ae-9e45-857fe4180681")
                IAppServiceRequestReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::ApplicationModel::AppService::IAppServiceRequest** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::AppService::IAppServiceDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceRequestReceivedEventArgs = __uuidof(IAppServiceRequestReceivedEventArgs);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceResponse[] = L"Windows.ApplicationModel.AppService.IAppServiceResponse";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("8d503cec-9aa3-4e68-9559-9de63e372ce4")
                IAppServiceResponse : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::AppService::AppServiceResponseStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceResponse = __uuidof(IAppServiceResponse);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("88a2dcac-ad28-41b8-80bb-bdf1b2169e19")
                IAppServiceTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallerPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppServiceConnection(
                        ABI::Windows::ApplicationModel::AppService::IAppServiceConnection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceTriggerDetails = __uuidof(IAppServiceTriggerDetails);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails2[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("e83d54b2-28cc-43f2-b465-c0482e59e2dc")
                IAppServiceTriggerDetails2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRemoteSystemConnection(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceTriggerDetails2 = __uuidof(IAppServiceTriggerDetails2);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails3[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("fbd71e21-7939-4e68-9e3c-7780147aabb6")
                IAppServiceTriggerDetails3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CheckCallerForCapabilityAsync(
                        HSTRING capabilityName,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceTriggerDetails3 = __uuidof(IAppServiceTriggerDetails3);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails4[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("1185b180-8861-5e30-ab55-1cf4d08bbf6d")
                IAppServiceTriggerDetails4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CallerRemoteConnectionToken(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppServiceTriggerDetails4 = __uuidof(IAppServiceTriggerDetails4);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IStatelessAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.StatelessAppServiceResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IStatelessAppServiceResponse[] = L"Windows.ApplicationModel.AppService.IStatelessAppServiceResponse";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                MIDL_INTERFACE("43754af7-a9ec-52fe-82e7-939b68dc9388")
                IStatelessAppServiceResponse : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::AppService::StatelessAppServiceResponseStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStatelessAppServiceResponse = __uuidof(IStatelessAppServiceResponse);
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppService.IAppServiceCatalogStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceCatalog[] = L"Windows.ApplicationModel.AppService.AppServiceCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs[] = L"Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppService.IAppServiceConnectionStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceConnection ** Default Interface **
 *    Windows.ApplicationModel.AppService.IAppServiceConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceConnection[] = L"Windows.ApplicationModel.AppService.AppServiceConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceDeferral[] = L"Windows.ApplicationModel.AppService.AppServiceDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceRequest[] = L"Windows.ApplicationModel.AppService.AppServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs[] = L"Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceResponse[] = L"Windows.ApplicationModel.AppService.AppServiceResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails ** Default Interface **
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceTriggerDetails[] = L"Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.StatelessAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IStatelessAppServiceResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_StatelessAppServiceResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_StatelessAppServiceResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_StatelessAppServiceResponse[] = L"Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2 __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2 __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3 __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4 __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceConnectionStatus __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceConnectionStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceConnectionStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceResponseStatus __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceResponseStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceResponseStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppInfo __FIIterator_1_Windows__CApplicationModel__CAppInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppInfo;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppInfoVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppInfo __FIIterable_1_Windows__CApplicationModel__CAppInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppInfo;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppInfo* This,
        __FIIterator_1_Windows__CApplicationModel__CAppInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppInfoVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppInfo __FIVectorView_1_Windows__CApplicationModel__CAppInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest;

#endif // ____x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceClosedStatus __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceClosedStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppService_CStatelessAppServiceResponseStatus __x_ABI_CWindows_CApplicationModel_CAppService_CStatelessAppServiceResponseStatus;

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceClosedStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceClosedStatus
{
    AppServiceClosedStatus_Completed = 0,
    AppServiceClosedStatus_Canceled = 1,
    AppServiceClosedStatus_ResourceLimitsExceeded = 2,
    AppServiceClosedStatus_Unknown = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceConnectionStatus
{
    AppServiceConnectionStatus_Success = 0,
    AppServiceConnectionStatus_AppNotInstalled = 1,
    AppServiceConnectionStatus_AppUnavailable = 2,
    AppServiceConnectionStatus_AppServiceUnavailable = 3,
    AppServiceConnectionStatus_Unknown = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppServiceConnectionStatus_RemoteSystemUnavailable = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppServiceConnectionStatus_RemoteSystemNotSupportedByApp = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppServiceConnectionStatus_NotAuthorized = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceConnectionStatus_AuthenticationError = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceConnectionStatus_NetworkNotAvailable = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceConnectionStatus_DisabledByPolicy = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceConnectionStatus_WebServiceUnavailable = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.AppServiceResponseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceResponseStatus
{
    AppServiceResponseStatus_Success = 0,
    AppServiceResponseStatus_Failure = 1,
    AppServiceResponseStatus_ResourceLimitsExceeded = 2,
    AppServiceResponseStatus_Unknown = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppServiceResponseStatus_RemoteSystemUnavailable = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppServiceResponseStatus_MessageSizeTooLarge = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceResponseStatus_AppUnavailable = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceResponseStatus_AuthenticationError = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceResponseStatus_NetworkNotAvailable = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceResponseStatus_DisabledByPolicy = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    AppServiceResponseStatus_WebServiceUnavailable = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CApplicationModel_CAppService_CStatelessAppServiceResponseStatus
{
    StatelessAppServiceResponseStatus_Success = 0,
    StatelessAppServiceResponseStatus_AppNotInstalled = 1,
    StatelessAppServiceResponseStatus_AppUnavailable = 2,
    StatelessAppServiceResponseStatus_AppServiceUnavailable = 3,
    StatelessAppServiceResponseStatus_RemoteSystemUnavailable = 4,
    StatelessAppServiceResponseStatus_RemoteSystemNotSupportedByApp = 5,
    StatelessAppServiceResponseStatus_NotAuthorized = 6,
    StatelessAppServiceResponseStatus_ResourceLimitsExceeded = 7,
    StatelessAppServiceResponseStatus_MessageSizeTooLarge = 8,
    StatelessAppServiceResponseStatus_Failure = 9,
    StatelessAppServiceResponseStatus_Unknown = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    StatelessAppServiceResponseStatus_AuthenticationError = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    StatelessAppServiceResponseStatus_NetworkNotAvailable = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    StatelessAppServiceResponseStatus_DisabledByPolicy = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    StatelessAppServiceResponseStatus_WebServiceUnavailable = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceCatalogStatics[] = L"Windows.ApplicationModel.AppService.IAppServiceCatalogStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAppServiceProvidersAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics* This,
        HSTRING appServiceName,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppInfo** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_FindAppServiceProvidersAsync(This, appServiceName, operation) \
    ((This)->lpVtbl->FindAppServiceProvidersAsync(This, appServiceName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceClosedEventArgs[] = L"Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceClosedStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnection[] = L"Windows.ApplicationModel.AppService.IAppServiceConnection";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppServiceName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AppServiceName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus** operation);
    HRESULT (STDMETHODCALLTYPE* SendMessageAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* message,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponse** operation);
    HRESULT (STDMETHODCALLTYPE* add_RequestReceived)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceRequestReceivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RequestReceived)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ServiceClosed)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppService__CAppServiceConnection_Windows__CApplicationModel__CAppService__CAppServiceClosedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ServiceClosed)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_get_AppServiceName(This, value) \
    ((This)->lpVtbl->get_AppServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_put_AppServiceName(This, value) \
    ((This)->lpVtbl->put_AppServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_put_PackageFamilyName(This, value) \
    ((This)->lpVtbl->put_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_OpenAsync(This, operation) \
    ((This)->lpVtbl->OpenAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_SendMessageAsync(This, message, operation) \
    ((This)->lpVtbl->SendMessageAsync(This, message, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_add_RequestReceived(This, handler, token) \
    ((This)->lpVtbl->add_RequestReceived(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_remove_RequestReceived(This, token) \
    ((This)->lpVtbl->remove_RequestReceived(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_add_ServiceClosed(This, handler, token) \
    ((This)->lpVtbl->add_ServiceClosed(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_remove_ServiceClosed(This, token) \
    ((This)->lpVtbl->remove_ServiceClosed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnection2[] = L"Windows.ApplicationModel.AppService.IAppServiceConnection2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenRemoteAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* remoteSystemConnectionRequest,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceConnectionStatus** operation);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* put_User)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2* This,
        __x_ABI_CWindows_CSystem_CIUser* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_OpenRemoteAsync(This, remoteSystemConnectionRequest, operation) \
    ((This)->lpVtbl->OpenRemoteAsync(This, remoteSystemConnectionRequest, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_put_User(This, value) \
    ((This)->lpVtbl->put_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceConnectionStatics[] = L"Windows.ApplicationModel.AppService.IAppServiceConnectionStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendStatelessMessageAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* connection,
        __x_ABI_CWindows_CSystem_CRemoteSystems_CIRemoteSystemConnectionRequest* connectionRequest,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* message,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CStatelessAppServiceResponse** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_SendStatelessMessageAsync(This, connection, connectionRequest, message, operation) \
    ((This)->lpVtbl->SendStatelessMessageAsync(This, connection, connectionRequest, message, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceDeferral[] = L"Windows.ApplicationModel.AppService.IAppServiceDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceRequest[] = L"Windows.ApplicationModel.AppService.IAppServiceRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* SendResponseAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* message,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppService__CAppServiceResponseStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_SendResponseAsync(This, message, operation) \
    ((This)->lpVtbl->SendResponseAsync(This, message, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceRequestReceivedEventArgs[] = L"Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceRequestReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceResponse[] = L"Windows.ApplicationModel.AppService.IAppServiceResponse";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppService_CAppServiceResponseStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponseVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CallerPackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppServiceConnection)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_get_CallerPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_CallerPackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_get_AppServiceConnection(This, value) \
    ((This)->lpVtbl->get_AppServiceConnection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails2[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRemoteSystemConnection)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_get_IsRemoteSystemConnection(This, value) \
    ((This)->lpVtbl->get_IsRemoteSystemConnection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails3[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CheckCallerForCapabilityAsync)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3* This,
        HSTRING capabilityName,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_CheckCallerForCapabilityAsync(This, capabilityName, operation) \
    ((This)->lpVtbl->CheckCallerForCapabilityAsync(This, capabilityName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IAppServiceTriggerDetails4[] = L"Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CallerRemoteConnectionToken)(__x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_get_CallerRemoteConnectionToken(This, value) \
    ((This)->lpVtbl->get_CallerRemoteConnectionToken(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceTriggerDetails4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.AppService.IStatelessAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppService.StatelessAppServiceResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_AppService_IStatelessAppServiceResponse[] = L"Windows.ApplicationModel.AppService.IStatelessAppServiceResponse";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppService_CStatelessAppServiceResponseStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponseVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppService_CIStatelessAppServiceResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppService.IAppServiceCatalogStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceCatalog[] = L"Windows.ApplicationModel.AppService.AppServiceCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceClosedEventArgs[] = L"Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.AppService.IAppServiceConnectionStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceConnection ** Default Interface **
 *    Windows.ApplicationModel.AppService.IAppServiceConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceConnection[] = L"Windows.ApplicationModel.AppService.AppServiceConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceDeferral[] = L"Windows.ApplicationModel.AppService.AppServiceDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceRequest[] = L"Windows.ApplicationModel.AppService.AppServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceRequestReceivedEventArgs[] = L"Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceResponse[] = L"Windows.ApplicationModel.AppService.AppServiceResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.AppServiceTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails ** Default Interface **
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3
 *    Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_AppServiceTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_AppServiceTriggerDetails[] = L"Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppService.StatelessAppServiceResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.AppService.IStatelessAppServiceResponse ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppService_StatelessAppServiceResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppService_StatelessAppServiceResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppService_StatelessAppServiceResponse[] = L"Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
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
#endif // __windows2Eapplicationmodel2Eappservice_p_h__

#endif // __windows2Eapplicationmodel2Eappservice_h__
