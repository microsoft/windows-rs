
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
#ifndef __windows2Eservices2Ecortana_h__
#define __windows2Eservices2Ecortana_h__
#ifndef __windows2Eservices2Ecortana_p_h__
#define __windows2Eservices2Ecortana_p_h__


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
#include "Windows.ApplicationModel.DataTransfer.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaActionableInsights;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights ABI::Windows::Services::Cortana::ICortanaActionableInsights

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaActionableInsightsOptions;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions ABI::Windows::Services::Cortana::ICortanaActionableInsightsOptions

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaActionableInsightsStatics;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics ABI::Windows::Services::Cortana::ICortanaActionableInsightsStatics

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaPermissionsManager;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager ABI::Windows::Services::Cortana::ICortanaPermissionsManager

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaPermissionsManagerStatics;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics ABI::Windows::Services::Cortana::ICortanaPermissionsManagerStatics

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaSettings;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings ABI::Windows::Services::Cortana::ICortanaSettings

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                interface ICortanaSettingsStatics;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics ABI::Windows::Services::Cortana::ICortanaSettingsStatics

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__

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
        namespace Services {
            namespace Cortana {
                typedef enum CortanaPermissionsChangeResult : int CortanaPermissionsChangeResult;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("838a3dd0-f0a3-508f-846a-d3c19e4fe7a0"))
IAsyncOperation<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult> : IAsyncOperation_impl<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Services.Cortana.CortanaPermissionsChangeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult> __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_t;
#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec1c6586-5e0d-5bc0-b84f-20052c5ac7a9"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Services.Cortana.CortanaPermissionsChangeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Services::Cortana::CortanaPermissionsChangeResult> __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                typedef enum CortanaPermission : int CortanaPermission;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_USE
#define DEF___FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0f1ac33c-511a-52e8-af09-d89f7004e8c5"))
IIterator<enum ABI::Windows::Services::Cortana::CortanaPermission> : IIterator_impl<enum ABI::Windows::Services::Cortana::CortanaPermission>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Services.Cortana.CortanaPermission>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Services::Cortana::CortanaPermission> __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_t;
#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_USE
#define DEF___FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("36a12eae-2e24-5e07-bfd0-344a92990916"))
IIterable<enum ABI::Windows::Services::Cortana::CortanaPermission> : IIterable_impl<enum ABI::Windows::Services::Cortana::CortanaPermission>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Services.Cortana.CortanaPermission>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Services::Cortana::CortanaPermission> __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_t;
#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                class DataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace DataTransfer {
                interface IDataPackage;
            } /* DataTransfer */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage ABI::Windows::ApplicationModel::DataTransfer::IDataPackage

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

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
        namespace Services {
            namespace Cortana {
                class CortanaActionableInsights;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                class CortanaActionableInsightsOptions;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                class CortanaPermissionsManager;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                class CortanaSettings;
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Services.Cortana.CortanaPermission
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("CortanaPermission is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                CortanaPermission : int
                {
                    CortanaPermission_BrowsingHistory = 0,
                    CortanaPermission_Calendar = 1,
                    CortanaPermission_CallHistory = 2,
                    CortanaPermission_Contacts = 3,
                    CortanaPermission_Email = 4,
                    CortanaPermission_InputPersonalization = 5,
                    CortanaPermission_Location = 6,
                    CortanaPermission_Messaging = 7,
                    CortanaPermission_Microphone = 8,
                    CortanaPermission_Personalization = 9,
                    CortanaPermission_PhoneCall = 10,
                };
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Cortana.CortanaPermissionsChangeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("CortanaPermissionsChangeResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                CortanaPermissionsChangeResult : int
                {
                    CortanaPermissionsChangeResult_Success = 0,
                    CortanaPermissionsChangeResult_Unavailable = 1,
                    CortanaPermissionsChangeResult_DisabledByPolicy = 2,
                };
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsights
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsights
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsights[] = L"Windows.Services.Cortana.ICortanaActionableInsights";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("951ec6b1-fc83-586d-8b84-2452c8981625")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ICortanaActionableInsights : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE IsAvailableAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsForImageAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* imageStream,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsForImageWithOptionsAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* imageStream,
                        ABI::Windows::Services::Cortana::ICortanaActionableInsightsOptions* options,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsForTextAsync(
                        HSTRING text,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsForTextWithOptionsAsync(
                        HSTRING text,
                        ABI::Windows::Services::Cortana::ICortanaActionableInsightsOptions* options,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsAsync(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage* datapackage,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ShowInsightsWithOptionsAsync(
                        ABI::Windows::ApplicationModel::DataTransfer::IDataPackage* datapackage,
                        ABI::Windows::Services::Cortana::ICortanaActionableInsightsOptions* options,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaActionableInsights = __uuidof(ICortanaActionableInsights);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsightsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsightsOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsightsOptions[] = L"Windows.Services.Cortana.ICortanaActionableInsightsOptions";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("aac2bbcf-9782-5420-b81e-7ae56af31815")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ICortanaActionableInsightsOptions : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_ContentSourceWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE put_ContentSourceWebLink(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_SurroundingText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE put_SurroundingText(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaActionableInsightsOptions = __uuidof(ICortanaActionableInsightsOptions);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsightsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsights
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsightsStatics[] = L"Windows.Services.Cortana.ICortanaActionableInsightsStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("b5ded412-9d2f-5cb5-9b05-356a0b836c10")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ICortanaActionableInsightsStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Services::Cortana::ICortanaActionableInsights** result
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Services::Cortana::ICortanaActionableInsights** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaActionableInsightsStatics = __uuidof(ICortanaActionableInsightsStatics);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaPermissionsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaPermissionsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaPermissionsManager[] = L"Windows.Services.Cortana.ICortanaPermissionsManager";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("191330e0-8695-438a-9545-3da4e822ddb4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ICortanaPermissionsManager : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE ArePermissionsGrantedAsync(
                        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
                        __FIAsyncOperation_1_boolean** getGrantedPermissionsOperation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GrantPermissionsAsync(
                        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
                        __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult** grantOperation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE RevokePermissionsAsync(
                        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
                        __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult** revokeOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaPermissionsManager = __uuidof(ICortanaPermissionsManager);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaPermissionsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaPermissionsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaPermissionsManagerStatics[] = L"Windows.Services.Cortana.ICortanaPermissionsManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("76b1e67a-b045-4414-9d6d-2ad3a5fe3a7e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("ICortanaPermissionsManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ICortanaPermissionsManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("ICortanaPermissionsManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Services::Cortana::ICortanaPermissionsManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaPermissionsManagerStatics = __uuidof(ICortanaPermissionsManagerStatics);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaSettings
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaSettings
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaSettings[] = L"Windows.Services.Cortana.ICortanaSettings";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("54d571a7-8062-40f4-abe7-dedfd697b019")
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                ICortanaSettings : public IInspectable
                {
                public:
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE get_HasUserConsentToVoiceActivation(
                        boolean* value
                        ) = 0;
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE get_IsVoiceActivationEnabled(
                        boolean* value
                        ) = 0;
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE put_IsVoiceActivationEnabled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaSettings = __uuidof(ICortanaSettings);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaSettings;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaSettingsStatics
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaSettings
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaSettingsStatics[] = L"Windows.Services.Cortana.ICortanaSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace Services {
            namespace Cortana {
                MIDL_INTERFACE("8b2ccd7e-2ec0-446d-9285-33f07ce8ac04")
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                ICortanaSettingsStatics : public IInspectable
                {
                public:
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* value
                        ) = 0;
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Services::Cortana::ICortanaSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICortanaSettingsStatics = __uuidof(ICortanaSettingsStatics);
            } /* Cortana */
        } /* Services */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Cortana.CortanaActionableInsights
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaActionableInsightsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaActionableInsights ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsights_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsights_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaActionableInsights[] = L"Windows.Services.Cortana.CortanaActionableInsights";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Services.Cortana.CortanaActionableInsightsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaActionableInsightsOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsightsOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsightsOptions_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaActionableInsightsOptions[] = L"Windows.Services.Cortana.CortanaActionableInsightsOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Services.Cortana.CortanaPermissionsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaPermissionsManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaPermissionsManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaPermissionsManager_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaPermissionsManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaPermissionsManager[] = L"Windows.Services.Cortana.CortanaPermissionsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Cortana.CortanaSettings
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaSettingsStatics interface starting with version 3.0 of the Windows.System.SystemManagementContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaSettings_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaSettings_DEFINED
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
DEPRECATED("CortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaSettings[] = L"Windows.Services.Cortana.CortanaSettings";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaSettings __x_ABI_CWindows_CServices_CCortana_CICortanaSettings;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics;

#endif // ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CServices_CCortana_CCortanaPermissionsChangeResult __x_ABI_CWindows_CServices_CCortana_CCortanaPermissionsChangeResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult;

typedef struct __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        enum __x_ABI_CWindows_CServices_CCortana_CCortanaPermissionsChangeResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl;

interface __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* This,
        __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef enum __x_ABI_CWindows_CServices_CCortana_CCortanaPermission __x_ABI_CWindows_CServices_CCortana_CCortanaPermission;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CServices__CCortana__CCortanaPermission;

typedef struct __FIIterator_1_Windows__CServices__CCortana__CCortanaPermissionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        enum __x_ABI_CWindows_CServices_CCortana_CCortanaPermission* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CServices__CCortana__CCortanaPermission* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CServices_CCortana_CCortanaPermission* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CServices__CCortana__CCortanaPermissionVtbl;

interface __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission
{
    CONST_VTBL struct __FIIterator_1_Windows__CServices__CCortana__CCortanaPermissionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CServices__CCortana__CCortanaPermission;

typedef struct __FIIterable_1_Windows__CServices__CCortana__CCortanaPermissionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* This,
        __FIIterator_1_Windows__CServices__CCortana__CCortanaPermission** result);

    END_INTERFACE
} __FIIterable_1_Windows__CServices__CCortana__CCortanaPermissionVtbl;

interface __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission
{
    CONST_VTBL struct __FIIterable_1_Windows__CServices__CCortana__CCortanaPermissionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CServices__CCortana__CCortanaPermission_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

/*
 *
 * Struct Windows.Services.Cortana.CortanaPermission
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaPermission is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CCortanaPermission
{
    CortanaPermission_BrowsingHistory = 0,
    CortanaPermission_Calendar = 1,
    CortanaPermission_CallHistory = 2,
    CortanaPermission_Contacts = 3,
    CortanaPermission_Email = 4,
    CortanaPermission_InputPersonalization = 5,
    CortanaPermission_Location = 6,
    CortanaPermission_Messaging = 7,
    CortanaPermission_Microphone = 8,
    CortanaPermission_Personalization = 9,
    CortanaPermission_PhoneCall = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Services.Cortana.CortanaPermissionsChangeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaPermissionsChangeResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CCortanaPermissionsChangeResult
{
    CortanaPermissionsChangeResult_Success = 0,
    CortanaPermissionsChangeResult_Unavailable = 1,
    CortanaPermissionsChangeResult_DisabledByPolicy = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsights
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsights
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsights[] = L"Windows.Services.Cortana.ICortanaActionableInsights";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* IsAvailableAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __FIAsyncOperation_1_boolean** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsForImageAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* imageStream,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsForImageWithOptionsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* imageStream,
        __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsForTextAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        HSTRING text,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsForTextWithOptionsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        HSTRING text,
        __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* datapackage,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ShowInsightsWithOptionsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage* datapackage,
        __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_IsAvailableAsync(This, operation) \
    ((This)->lpVtbl->IsAvailableAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsForImageAsync(This, imageStream, operation) \
    ((This)->lpVtbl->ShowInsightsForImageAsync(This, imageStream, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsForImageWithOptionsAsync(This, imageStream, options, operation) \
    ((This)->lpVtbl->ShowInsightsForImageWithOptionsAsync(This, imageStream, options, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsForTextAsync(This, text, operation) \
    ((This)->lpVtbl->ShowInsightsForTextAsync(This, text, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsForTextWithOptionsAsync(This, text, options, operation) \
    ((This)->lpVtbl->ShowInsightsForTextWithOptionsAsync(This, text, options, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsAsync(This, datapackage, operation) \
    ((This)->lpVtbl->ShowInsightsAsync(This, datapackage, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_ShowInsightsWithOptionsAsync(This, datapackage, options, operation) \
    ((This)->lpVtbl->ShowInsightsWithOptionsAsync(This, datapackage, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsightsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsightsOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsightsOptions[] = L"Windows.Services.Cortana.ICortanaActionableInsightsOptions";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_ContentSourceWebLink)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* put_ContentSourceWebLink)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_SurroundingText)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* put_SurroundingText)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptionsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_get_ContentSourceWebLink(This, value) \
    ((This)->lpVtbl->get_ContentSourceWebLink(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_put_ContentSourceWebLink(This, value) \
    ((This)->lpVtbl->put_ContentSourceWebLink(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_get_SurroundingText(This, value) \
    ((This)->lpVtbl->get_SurroundingText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_put_SurroundingText(This, value) \
    ((This)->lpVtbl->put_SurroundingText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaActionableInsightsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaActionableInsights
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaActionableInsightsStatics[] = L"Windows.Services.Cortana.ICortanaActionableInsightsStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsights** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStaticsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaActionableInsightsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaPermissionsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaPermissionsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaPermissionsManager[] = L"Windows.Services.Cortana.ICortanaPermissionsManager";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        boolean* result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* ArePermissionsGrantedAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
        __FIAsyncOperation_1_boolean** getGrantedPermissionsOperation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GrantPermissionsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
        __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult** grantOperation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* RevokePermissionsAsync)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager* This,
        __FIIterable_1_Windows__CServices__CCortana__CCortanaPermission* permissions,
        __FIAsyncOperation_1_Windows__CServices__CCortana__CCortanaPermissionsChangeResult** revokeOperation);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_ArePermissionsGrantedAsync(This, permissions, getGrantedPermissionsOperation) \
    ((This)->lpVtbl->ArePermissionsGrantedAsync(This, permissions, getGrantedPermissionsOperation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_GrantPermissionsAsync(This, permissions, grantOperation) \
    ((This)->lpVtbl->GrantPermissionsAsync(This, permissions, grantOperation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_RevokePermissionsAsync(This, permissions, revokeOperation) \
    ((This)->lpVtbl->RevokePermissionsAsync(This, permissions, revokeOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaPermissionsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaPermissionsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaPermissionsManagerStatics[] = L"Windows.Services.Cortana.ICortanaPermissionsManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("ICortanaPermissionsManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics* This,
        __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStaticsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("ICortanaPermissionsManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaPermissionsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaSettings
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaSettings
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaSettings[] = L"Windows.Services.Cortana.ICortanaSettings";
typedef struct
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        TrustLevel* trustLevel);
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_HasUserConsentToVoiceActivation)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        boolean* value);
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_IsVoiceActivationEnabled)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        boolean* value);
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_IsVoiceActivationEnabled)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettings* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_get_HasUserConsentToVoiceActivation(This, value) \
    ((This)->lpVtbl->get_HasUserConsentToVoiceActivation(This, value))

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_get_IsVoiceActivationEnabled(This, value) \
    ((This)->lpVtbl->get_IsVoiceActivationEnabled(This, value))

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettings_put_IsVoiceActivationEnabled(This, value) \
    ((This)->lpVtbl->put_IsVoiceActivationEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaSettings;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Services.Cortana.ICortanaSettingsStatics
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Services.Cortana.CortanaSettings
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Services_Cortana_ICortanaSettingsStatics[] = L"Windows.Services.Cortana.ICortanaSettingsStatics";
typedef struct
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        boolean* value);
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics* This,
        __x_ABI_CWindows_CServices_CCortana_CICortanaSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStaticsVtbl;

interface __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_IsSupported(This, value) \
    ((This)->lpVtbl->IsSupported(This, value))

#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
    DEPRECATED("ICortanaSettingsStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CServices_CCortana_CICortanaSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Services.Cortana.CortanaActionableInsights
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaActionableInsightsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaActionableInsights ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsights_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsights_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsights is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaActionableInsights[] = L"Windows.Services.Cortana.CortanaActionableInsights";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Services.Cortana.CortanaActionableInsightsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaActionableInsightsOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsightsOptions_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaActionableInsightsOptions_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaActionableInsightsOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaActionableInsightsOptions[] = L"Windows.Services.Cortana.CortanaActionableInsightsOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Services.Cortana.CortanaPermissionsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaPermissionsManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaPermissionsManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaPermissionsManager_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaPermissionsManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("CortanaPermissionsManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaPermissionsManager[] = L"Windows.Services.Cortana.CortanaPermissionsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Services.Cortana.CortanaSettings
 *
 * Introduced to Windows.System.SystemManagementContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Services.Cortana.ICortanaSettingsStatics interface starting with version 3.0 of the Windows.System.SystemManagementContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Services.Cortana.ICortanaSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Services_Cortana_CortanaSettings_DEFINED
#define RUNTIMECLASS_Windows_Services_Cortana_CortanaSettings_DEFINED
#if WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
DEPRECATED("CortanaSettings is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Services_Cortana_CortanaSettings[] = L"Windows.Services.Cortana.CortanaSettings";
#endif
#endif // WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eservices2Ecortana_p_h__

#endif // __windows2Eservices2Ecortana_h__
