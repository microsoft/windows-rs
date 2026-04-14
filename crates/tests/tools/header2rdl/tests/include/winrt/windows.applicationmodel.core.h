
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
#ifndef __windows2Eapplicationmodel2Ecore_h__
#define __windows2Eapplicationmodel2Ecore_h__
#ifndef __windows2Eapplicationmodel2Ecore_p_h__
#define __windows2Eapplicationmodel2Ecore_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.h"
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.System.h"
#include "Windows.UI.Core.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry ABI::Windows::ApplicationModel::Core::IAppListEntry

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry2;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2 ABI::Windows::ApplicationModel::Core::IAppListEntry2

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry3;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3 ABI::Windows::ApplicationModel::Core::IAppListEntry3

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry4;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4 ABI::Windows::ApplicationModel::Core::IAppListEntry4

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplication;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication ABI::Windows::ApplicationModel::Core::ICoreApplication

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplication2;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2 ABI::Windows::ApplicationModel::Core::ICoreApplication2

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplication3;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3 ABI::Windows::ApplicationModel::Core::ICoreApplication3

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationExit;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit ABI::Windows::ApplicationModel::Core::ICoreApplicationExit

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationUnhandledError;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError ABI::Windows::ApplicationModel::Core::ICoreApplicationUnhandledError

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationUseCount;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount ABI::Windows::ApplicationModel::Core::ICoreApplicationUseCount

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView ABI::Windows::ApplicationModel::Core::ICoreApplicationView

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView2;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2 ABI::Windows::ApplicationModel::Core::ICoreApplicationView2

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView3;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3 ABI::Windows::ApplicationModel::Core::ICoreApplicationView3

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView5;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5 ABI::Windows::ApplicationModel::Core::ICoreApplicationView5

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView6;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6 ABI::Windows::ApplicationModel::Core::ICoreApplicationView6

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationViewTitleBar;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar ABI::Windows::ApplicationModel::Core::ICoreApplicationViewTitleBar

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreImmersiveApplication;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication ABI::Windows::ApplicationModel::Core::ICoreImmersiveApplication

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreImmersiveApplication2;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2 ABI::Windows::ApplicationModel::Core::ICoreImmersiveApplication2

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreImmersiveApplication3;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3 ABI::Windows::ApplicationModel::Core::ICoreImmersiveApplication3

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IFrameworkView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView ABI::Windows::ApplicationModel::Core::IFrameworkView

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IFrameworkViewSource;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource ABI::Windows::ApplicationModel::Core::IFrameworkViewSource

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IHostedViewClosingEventArgs;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs ABI::Windows::ApplicationModel::Core::IHostedViewClosingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IUnhandledError;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError ABI::Windows::ApplicationModel::Core::IUnhandledError

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IUnhandledErrorDetectedEventArgs;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs ABI::Windows::ApplicationModel::Core::IUnhandledErrorDetectedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__

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
            namespace Core {
                typedef enum AppRestartFailureReason : int AppRestartFailureReason;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0938905d-54c0-572f-8451-4bfd2b52edda"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Core.AppRestartFailureReason>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason> __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dcec478a-9f27-5c5d-afdb-c91aee4f1f02"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Core.AppRestartFailureReason>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Core::AppRestartFailureReason> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class CoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f5f6944-264b-5868-809e-c7ac1ac5edad"))
IIterator<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::ICoreApplicationView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Core.CoreApplicationView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t;
#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("32bc12d1-2653-5a41-a55e-88a12af2026a"))
IIterable<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::ICoreApplicationView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Core.CoreApplicationView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t;
#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de9e16c4-1b7c-5126-b1d8-7cd04f13bd08"))
IVectorView<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::ICoreApplicationView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Core.CoreApplicationView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Core::CoreApplicationView*> __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t;
#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                class BackgroundActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IBackgroundActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IBackgroundActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("49a07732-e7b8-5c5b-9de7-22e33cb97004"))
IEventHandler<ABI::Windows::ApplicationModel::Activation::BackgroundActivatedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Activation::BackgroundActivatedEventArgs*, ABI::Windows::ApplicationModel::Activation::IBackgroundActivatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::Activation::BackgroundActivatedEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class UnhandledErrorDetectedEventArgs;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f68bc421-6b54-559b-9cdd-489aad0bd41d"))
IEventHandler<ABI::Windows::ApplicationModel::Core::UnhandledErrorDetectedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::UnhandledErrorDetectedEventArgs*, ABI::Windows::ApplicationModel::Core::IUnhandledErrorDetectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::Core::UnhandledErrorDetectedEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class EnteredBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IEnteredBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs ABI::Windows::ApplicationModel::IEnteredBackgroundEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e0739c32-fc14-5361-a8b3-0809699fbcbd"))
IEventHandler<ABI::Windows::ApplicationModel::EnteredBackgroundEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::EnteredBackgroundEventArgs*, ABI::Windows::ApplicationModel::IEnteredBackgroundEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.EnteredBackgroundEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::EnteredBackgroundEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class LeavingBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ILeavingBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs ABI::Windows::ApplicationModel::ILeavingBackgroundEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9b6171c2-abb2-5194-afc0-cef167c424eb"))
IEventHandler<ABI::Windows::ApplicationModel::LeavingBackgroundEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::LeavingBackgroundEventArgs*, ABI::Windows::ApplicationModel::ILeavingBackgroundEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.LeavingBackgroundEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::LeavingBackgroundEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class SuspendingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs ABI::Windows::ApplicationModel::ISuspendingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("338579bf-1a35-5cc4-a622-a6f384fd892c"))
IEventHandler<ABI::Windows::ApplicationModel::SuspendingEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::SuspendingEventArgs*, ABI::Windows::ApplicationModel::ISuspendingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.ApplicationModel.SuspendingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::ApplicationModel::SuspendingEventArgs*> __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_t;
#define __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cf193a96-eb13-5e3b-8bdf-87b6efae8339"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::ICoreApplicationView*>, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Core.CoreApplicationView, Windows.ApplicationModel.Activation.IActivatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class HostedViewClosingEventArgs;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d9a3f433-9bcc-54d6-b3cf-7b01f026d4cd"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::HostedViewClosingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::ICoreApplicationView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::HostedViewClosingEventArgs*, ABI::Windows::ApplicationModel::Core::IHostedViewClosingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Core.CoreApplicationView, Windows.ApplicationModel.Core.HostedViewClosingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationView*, ABI::Windows::ApplicationModel::Core::HostedViewClosingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class CoreApplicationViewTitleBar;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28342e21-dad3-5e32-bae1-afe7b26c66fb"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationViewTitleBar*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::CoreApplicationViewTitleBar*, ABI::Windows::ApplicationModel::Core::ICoreApplicationViewTitleBar*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Core.CoreApplicationViewTitleBar, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Core::CoreApplicationViewTitleBar*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo ABI::Windows::ApplicationModel::IAppDisplayInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IGetActivationFactory;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIGetActivationFactory ABI::Windows::Foundation::IGetActivationFactory

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            class DispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IDispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIDispatcherQueue ABI::Windows::System::IDispatcherQueue

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

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
        namespace UI {
            namespace Core {
                class CoreDispatcher;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreDispatcher;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher ABI::Windows::UI::Core::ICoreDispatcher

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow ABI::Windows::UI::Core::ICoreWindow

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class UnhandledError;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Core.AppRestartFailureReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                enum AppRestartFailureReason : int
                {
                    AppRestartFailureReason_RestartPending = 0,
                    AppRestartFailureReason_NotInForeground = 1,
                    AppRestartFailureReason_InvalidUser = 2,
                    AppRestartFailureReason_Other = 3,
                };
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry[] = L"Windows.ApplicationModel.Core.IAppListEntry";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("ef00f07f-2108-490a-877a-8a9f17c25fad")
                IAppListEntry : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayInfo(
                        ABI::Windows::ApplicationModel::IAppDisplayInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LaunchAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppListEntry = __uuidof(IAppListEntry);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry2[] = L"Windows.ApplicationModel.Core.IAppListEntry2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("d0a618ad-bf35-42ac-ac06-86eeeb41d04b")
                IAppListEntry2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppListEntry2 = __uuidof(IAppListEntry2);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry3[] = L"Windows.ApplicationModel.Core.IAppListEntry3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("6099f28d-fc32-470a-bc69-4b061a76ef2e")
                IAppListEntry3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE LaunchForUserAsync(
                        ABI::Windows::System::IUser* user,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppListEntry3 = __uuidof(IAppListEntry3);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry4[] = L"Windows.ApplicationModel.Core.IAppListEntry4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("2a131ed2-56f5-487c-8697-5166f3b33da0")
                IAppListEntry4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppInfo(
                        ABI::Windows::ApplicationModel::IAppInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppListEntry4 = __uuidof(IAppListEntry4);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication[] = L"Windows.ApplicationModel.Core.ICoreApplication";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("0aacf7a4-5e1d-49df-8034-fb6a68bc5ed1")
                ICoreApplication : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Suspending(
                        __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Suspending(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Resuming(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Resuming(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentView(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Run(
                        ABI::Windows::ApplicationModel::Core::IFrameworkViewSource* viewSource
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RunWithActivationFactories(
                        ABI::Windows::Foundation::IGetActivationFactory* activationFactoryCallback
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplication = __uuidof(ICoreApplication);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication2[] = L"Windows.ApplicationModel.Core.ICoreApplication2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("998681fb-1ab6-4b7f-be4a-9a0645224c04")
                ICoreApplication2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_BackgroundActivated(
                        __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BackgroundActivated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LeavingBackground(
                        __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LeavingBackground(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnteredBackground(
                        __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnteredBackground(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnablePrelaunch(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplication2 = __uuidof(ICoreApplication2);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication3[] = L"Windows.ApplicationModel.Core.ICoreApplication3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("feec0d39-598b-4507-8a67-772632580a57")
                ICoreApplication3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestRestartAsync(
                        HSTRING launchArguments,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestRestartForUserAsync(
                        ABI::Windows::System::IUser* user,
                        HSTRING launchArguments,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplication3 = __uuidof(ICoreApplication3);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationExit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationExit[] = L"Windows.ApplicationModel.Core.ICoreApplicationExit";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("cf86461d-261e-4b72-9acd-44ed2ace6a29")
                ICoreApplicationExit : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Exit(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Exiting(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Exiting(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationExit = __uuidof(ICoreApplicationExit);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationUnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationUnhandledError[] = L"Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e")
                ICoreApplicationUnhandledError : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_UnhandledErrorDetected(
                        __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UnhandledErrorDetected(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationUnhandledError = __uuidof(ICoreApplicationUnhandledError);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationUseCount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationUseCount[] = L"Windows.ApplicationModel.Core.ICoreApplicationUseCount";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("518dc408-c077-475b-809e-0bc0c57e4b74")
                ICoreApplicationUseCount : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IncrementApplicationUseCount(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DecrementApplicationUseCount(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationUseCount = __uuidof(ICoreApplicationUseCount);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView[] = L"Windows.ApplicationModel.Core.ICoreApplicationView";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("638bb2db-451d-4661-b099-414f34ffb9f1")
                ICoreApplicationView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CoreWindow(
                        ABI::Windows::UI::Core::ICoreWindow** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Activated(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMain(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsHosted(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationView = __uuidof(ICoreApplicationView);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView2[] = L"Windows.ApplicationModel.Core.ICoreApplicationView2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("68eb7adf-917f-48eb-9aeb-7de53e086ab1")
                ICoreApplicationView2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Dispatcher(
                        ABI::Windows::UI::Core::ICoreDispatcher** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationView2 = __uuidof(ICoreApplicationView2);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView3[] = L"Windows.ApplicationModel.Core.ICoreApplicationView3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("07ebe1b3-a4cf-4550-ab70-b07e85330bc8")
                ICoreApplicationView3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsComponent(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TitleBar(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationViewTitleBar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HostedViewClosing(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HostedViewClosing(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationView3 = __uuidof(ICoreApplicationView3);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView5[] = L"Windows.ApplicationModel.Core.ICoreApplicationView5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("2bc095a8-8ef0-446d-9e60-3a3e0428c671")
                ICoreApplicationView5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationView5 = __uuidof(ICoreApplicationView5);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView6[] = L"Windows.ApplicationModel.Core.ICoreApplicationView6";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("c119d49a-0679-49ba-803f-b79c5cf34cca")
                ICoreApplicationView6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationView6 = __uuidof(ICoreApplicationView6);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationViewTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationViewTitleBar[] = L"Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("006d35e3-e1f1-431b-9508-29b96926ac53")
                ICoreApplicationViewTitleBar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ExtendViewIntoTitleBar(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendViewIntoTitleBar(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemOverlayLeftInset(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemOverlayRightInset(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LayoutMetricsChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LayoutMetricsChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsVisibleChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsVisibleChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreApplicationViewTitleBar = __uuidof(ICoreApplicationViewTitleBar);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("1ada0e3e-e4a2-4123-b451-dc96bf800419")
                ICoreImmersiveApplication : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Views(
                        __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateNewView(
                        HSTRING runtimeType,
                        HSTRING entryPoint,
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView** view
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainView(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreImmersiveApplication = __uuidof(ICoreImmersiveApplication);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication2[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("828e1e36-e9e3-4cfc-9b66-48b78ea9bb2c")
                ICoreImmersiveApplication2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateNewViewFromMainView(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView** view
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreImmersiveApplication2 = __uuidof(ICoreImmersiveApplication2);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication3[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("34a05b2f-ee0d-41e5-8314-cf10c91bf0af")
                ICoreImmersiveApplication3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateNewViewWithViewSource(
                        ABI::Windows::ApplicationModel::Core::IFrameworkViewSource* viewSource,
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView** view
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreImmersiveApplication3 = __uuidof(ICoreImmersiveApplication3);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IFrameworkView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IFrameworkView[] = L"Windows.ApplicationModel.Core.IFrameworkView";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("faab5cd0-8924-45ac-ad0f-a08fae5d0324")
                IFrameworkView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Initialize(
                        ABI::Windows::ApplicationModel::Core::ICoreApplicationView* applicationView
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetWindow(
                        ABI::Windows::UI::Core::ICoreWindow* window
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Load(
                        HSTRING entryPoint
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Run(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Uninitialize(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IFrameworkView = __uuidof(IFrameworkView);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IFrameworkViewSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IFrameworkViewSource[] = L"Windows.ApplicationModel.Core.IFrameworkViewSource";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("cd770614-65c4-426c-9494-34fc43554862")
                IFrameworkViewSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateView(
                        ABI::Windows::ApplicationModel::Core::IFrameworkView** viewProvider
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFrameworkViewSource = __uuidof(IFrameworkViewSource);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IHostedViewClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.HostedViewClosingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IHostedViewClosingEventArgs[] = L"Windows.ApplicationModel.Core.IHostedViewClosingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("d238943c-b24e-4790-acb5-3e4243c4ff87")
                IHostedViewClosingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHostedViewClosingEventArgs = __uuidof(IHostedViewClosingEventArgs);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IUnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.UnhandledError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IUnhandledError[] = L"Windows.ApplicationModel.Core.IUnhandledError";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("9459b726-53b5-4686-9eaf-fa8162dc3980")
                IUnhandledError : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Propagate(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnhandledError = __uuidof(IUnhandledError);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IUnhandledErrorDetectedEventArgs[] = L"Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                MIDL_INTERFACE("679ab78b-b336-4822-ac40-0d750f0b7a2b")
                IUnhandledErrorDetectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UnhandledError(
                        ABI::Windows::ApplicationModel::Core::IUnhandledError** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnhandledErrorDetectedEventArgs = __uuidof(IUnhandledErrorDetectedEventArgs);
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.AppListEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IAppListEntry ** Default Interface **
 *    Windows.ApplicationModel.Core.IAppListEntry2
 *    Windows.ApplicationModel.Core.IAppListEntry3
 *    Windows.ApplicationModel.Core.IAppListEntry4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_AppListEntry_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_AppListEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_AppListEntry[] = L"Windows.ApplicationModel.Core.AppListEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationExit interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationUnhandledError interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationUseCount interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplication_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplication_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplication[] = L"Windows.ApplicationModel.Core.CoreApplication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.ICoreApplicationView ** Default Interface **
 *    Windows.ApplicationModel.Core.ICoreApplicationView2
 *    Windows.ApplicationModel.Core.ICoreApplicationView3
 *    Windows.ApplicationModel.Core.ICoreApplicationView5
 *    Windows.ApplicationModel.Core.ICoreApplicationView6
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplicationView[] = L"Windows.ApplicationModel.Core.CoreApplicationView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar[] = L"Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.HostedViewClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IHostedViewClosingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_HostedViewClosingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_HostedViewClosingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_HostedViewClosingEventArgs[] = L"Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.UnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IUnhandledError ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledError_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_UnhandledError[] = L"Windows.ApplicationModel.Core.UnhandledError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs[] = L"Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2 __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3 __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4 __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6 __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2 __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3 __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CApplicationModel_CCore_CAppRestartFailureReason __x_ABI_CWindows_CApplicationModel_CCore_CAppRestartFailureReason;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        enum __x_ABI_CWindows_CApplicationModel_CCore_CAppRestartFailureReason* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReasonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

typedef struct __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

typedef struct __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        __FIIterator_1_Windows__CApplicationModel__CCore__CCoreApplicationView** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs;

typedef struct __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgsVtbl;

interface __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* sender,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* sender,
        __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIGetActivationFactory __x_ABI_CWindows_CFoundation_CIGetActivationFactory;

#endif // ____x_ABI_CWindows_CFoundation_CIGetActivationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher __x_ABI_CWindows_CUI_CCore_CICoreDispatcher;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow __x_ABI_CWindows_CUI_CCore_CICoreWindow;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

/*
 *
 * Struct Windows.ApplicationModel.Core.AppRestartFailureReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CApplicationModel_CCore_CAppRestartFailureReason
{
    AppRestartFailureReason_RestartPending = 0,
    AppRestartFailureReason_NotInForeground = 1,
    AppRestartFailureReason_InvalidUser = 2,
    AppRestartFailureReason_Other = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry[] = L"Windows.ApplicationModel.Core.IAppListEntry";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayInfo)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo** value);
    HRESULT (STDMETHODCALLTYPE* LaunchAsync)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_get_DisplayInfo(This, value) \
    ((This)->lpVtbl->get_DisplayInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_LaunchAsync(This, operation) \
    ((This)->lpVtbl->LaunchAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry2[] = L"Windows.ApplicationModel.Core.IAppListEntry2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry3[] = L"Windows.ApplicationModel.Core.IAppListEntry3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LaunchForUserAsync)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_LaunchForUserAsync(This, user, operation) \
    ((This)->lpVtbl->LaunchForUserAsync(This, user, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Core.IAppListEntry4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.AppListEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IAppListEntry4[] = L"Windows.ApplicationModel.Core.IAppListEntry4";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppInfo)(__x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_get_AppInfo(This, value) \
    ((This)->lpVtbl->get_AppInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication[] = L"Windows.ApplicationModel.Core.ICoreApplication";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* add_Suspending)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __FIEventHandler_1_Windows__CApplicationModel__CSuspendingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Suspending)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Resuming)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Resuming)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentView)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** value);
    HRESULT (STDMETHODCALLTYPE* Run)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* viewSource);
    HRESULT (STDMETHODCALLTYPE* RunWithActivationFactories)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication* This,
        __x_ABI_CWindows_CFoundation_CIGetActivationFactory* activationFactoryCallback);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_add_Suspending(This, handler, token) \
    ((This)->lpVtbl->add_Suspending(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_remove_Suspending(This, token) \
    ((This)->lpVtbl->remove_Suspending(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_add_Resuming(This, handler, token) \
    ((This)->lpVtbl->add_Resuming(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_remove_Resuming(This, token) \
    ((This)->lpVtbl->remove_Resuming(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_GetCurrentView(This, value) \
    ((This)->lpVtbl->GetCurrentView(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_Run(This, viewSource) \
    ((This)->lpVtbl->Run(This, viewSource))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_RunWithActivationFactories(This, activationFactoryCallback) \
    ((This)->lpVtbl->RunWithActivationFactories(This, activationFactoryCallback))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication2[] = L"Windows.ApplicationModel.Core.ICoreApplication2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_BackgroundActivated)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        __FIEventHandler_1_Windows__CApplicationModel__CActivation__CBackgroundActivatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BackgroundActivated)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LeavingBackground)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        __FIEventHandler_1_Windows__CApplicationModel__CLeavingBackgroundEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LeavingBackground)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnteredBackground)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        __FIEventHandler_1_Windows__CApplicationModel__CEnteredBackgroundEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnteredBackground)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* EnablePrelaunch)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_add_BackgroundActivated(This, handler, token) \
    ((This)->lpVtbl->add_BackgroundActivated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_remove_BackgroundActivated(This, token) \
    ((This)->lpVtbl->remove_BackgroundActivated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_add_LeavingBackground(This, handler, token) \
    ((This)->lpVtbl->add_LeavingBackground(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_remove_LeavingBackground(This, token) \
    ((This)->lpVtbl->remove_LeavingBackground(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_add_EnteredBackground(This, handler, token) \
    ((This)->lpVtbl->add_EnteredBackground(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_remove_EnteredBackground(This, token) \
    ((This)->lpVtbl->remove_EnteredBackground(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_EnablePrelaunch(This, value) \
    ((This)->lpVtbl->EnablePrelaunch(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplication3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplication3[] = L"Windows.ApplicationModel.Core.ICoreApplication3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestRestartAsync)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        HSTRING launchArguments,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation);
    HRESULT (STDMETHODCALLTYPE* RequestRestartForUserAsync)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING launchArguments,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_RequestRestartAsync(This, launchArguments, operation) \
    ((This)->lpVtbl->RequestRestartAsync(This, launchArguments, operation))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_RequestRestartForUserAsync(This, user, launchArguments, operation) \
    ((This)->lpVtbl->RequestRestartForUserAsync(This, user, launchArguments, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplication3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationExit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationExit[] = L"Windows.ApplicationModel.Core.ICoreApplicationExit";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExitVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Exit)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This);
    HRESULT (STDMETHODCALLTYPE* add_Exiting)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Exiting)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExitVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExitVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_Exit(This) \
    ((This)->lpVtbl->Exit(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_add_Exiting(This, handler, token) \
    ((This)->lpVtbl->add_Exiting(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_remove_Exiting(This, token) \
    ((This)->lpVtbl->remove_Exiting(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationExit_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationUnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationUnhandledError[] = L"Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_UnhandledErrorDetected)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        __FIEventHandler_1_Windows__CApplicationModel__CCore__CUnhandledErrorDetectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnhandledErrorDetected)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledErrorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_add_UnhandledErrorDetected(This, handler, token) \
    ((This)->lpVtbl->add_UnhandledErrorDetected(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_remove_UnhandledErrorDetected(This, token) \
    ((This)->lpVtbl->remove_UnhandledErrorDetected(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUnhandledError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationUseCount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationUseCount[] = L"Windows.ApplicationModel.Core.ICoreApplicationUseCount";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IncrementApplicationUseCount)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This);
    HRESULT (STDMETHODCALLTYPE* DecrementApplicationUseCount)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCountVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_IncrementApplicationUseCount(This) \
    ((This)->lpVtbl->IncrementApplicationUseCount(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_DecrementApplicationUseCount(This) \
    ((This)->lpVtbl->DecrementApplicationUseCount(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationUseCount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView[] = L"Windows.ApplicationModel.Core.ICoreApplicationView";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CoreWindow)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow** value);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IsMain)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHosted)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_get_CoreWindow(This, value) \
    ((This)->lpVtbl->get_CoreWindow(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_add_Activated(This, handler, token) \
    ((This)->lpVtbl->add_Activated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_remove_Activated(This, token) \
    ((This)->lpVtbl->remove_Activated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_get_IsMain(This, value) \
    ((This)->lpVtbl->get_IsMain(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_get_IsHosted(This, value) \
    ((This)->lpVtbl->get_IsHosted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView2[] = L"Windows.ApplicationModel.Core.ICoreApplicationView2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Dispatcher)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2* This,
        __x_ABI_CWindows_CUI_CCore_CICoreDispatcher** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_get_Dispatcher(This, value) \
    ((This)->lpVtbl->get_Dispatcher(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView3[] = L"Windows.ApplicationModel.Core.ICoreApplicationView3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsComponent)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_TitleBar)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar** value);
    HRESULT (STDMETHODCALLTYPE* add_HostedViewClosing)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationView_Windows__CApplicationModel__CCore__CHostedViewClosingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HostedViewClosing)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_get_IsComponent(This, value) \
    ((This)->lpVtbl->get_IsComponent(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_get_TitleBar(This, value) \
    ((This)->lpVtbl->get_TitleBar(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_add_HostedViewClosing(This, handler, token) \
    ((This)->lpVtbl->add_HostedViewClosing(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_remove_HostedViewClosing(This, token) \
    ((This)->lpVtbl->remove_HostedViewClosing(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView5[] = L"Windows.ApplicationModel.Core.ICoreApplicationView5";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationView6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationView6[] = L"Windows.ApplicationModel.Core.ICoreApplicationView6";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplicationViewTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreApplicationViewTitleBar[] = L"Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ExtendViewIntoTitleBar)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendViewIntoTitleBar)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemOverlayLeftInset)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemOverlayRightInset)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* add_LayoutMetricsChanged)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LayoutMetricsChanged)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_IsVisibleChanged)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCore__CCoreApplicationViewTitleBar_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsVisibleChanged)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBarVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_put_ExtendViewIntoTitleBar(This, value) \
    ((This)->lpVtbl->put_ExtendViewIntoTitleBar(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_get_ExtendViewIntoTitleBar(This, value) \
    ((This)->lpVtbl->get_ExtendViewIntoTitleBar(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_get_SystemOverlayLeftInset(This, value) \
    ((This)->lpVtbl->get_SystemOverlayLeftInset(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_get_SystemOverlayRightInset(This, value) \
    ((This)->lpVtbl->get_SystemOverlayRightInset(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_add_LayoutMetricsChanged(This, handler, token) \
    ((This)->lpVtbl->add_LayoutMetricsChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_remove_LayoutMetricsChanged(This, token) \
    ((This)->lpVtbl->remove_LayoutMetricsChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_add_IsVisibleChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsVisibleChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_remove_IsVisibleChanged(This, token) \
    ((This)->lpVtbl->remove_IsVisibleChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationViewTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplicationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Views)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        __FIVectorView_1_Windows__CApplicationModel__CCore__CCoreApplicationView** value);
    HRESULT (STDMETHODCALLTYPE* CreateNewView)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        HSTRING runtimeType,
        HSTRING entryPoint,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** view);
    HRESULT (STDMETHODCALLTYPE* get_MainView)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplicationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplicationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_get_Views(This, value) \
    ((This)->lpVtbl->get_Views(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_CreateNewView(This, runtimeType, entryPoint, view) \
    ((This)->lpVtbl->CreateNewView(This, runtimeType, entryPoint, view))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_get_MainView(This, value) \
    ((This)->lpVtbl->get_MainView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication2[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateNewViewFromMainView)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** view);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_CreateNewViewFromMainView(This, view) \
    ((This)->lpVtbl->CreateNewViewFromMainView(This, view))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.ICoreImmersiveApplication3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.CoreApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_ICoreImmersiveApplication3[] = L"Windows.ApplicationModel.Core.ICoreImmersiveApplication3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateNewViewWithViewSource)(__x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* viewSource,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView** view);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_CreateNewViewWithViewSource(This, viewSource, view) \
    ((This)->lpVtbl->CreateNewViewWithViewSource(This, viewSource, view))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CICoreImmersiveApplication3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IFrameworkView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IFrameworkView[] = L"Windows.ApplicationModel.Core.IFrameworkView";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Initialize)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* applicationView);
    HRESULT (STDMETHODCALLTYPE* SetWindow)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* window);
    HRESULT (STDMETHODCALLTYPE* Load)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This,
        HSTRING entryPoint);
    HRESULT (STDMETHODCALLTYPE* Run)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This);
    HRESULT (STDMETHODCALLTYPE* Uninitialize)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_Initialize(This, applicationView) \
    ((This)->lpVtbl->Initialize(This, applicationView))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_SetWindow(This, window) \
    ((This)->lpVtbl->SetWindow(This, window))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_Load(This, entryPoint) \
    ((This)->lpVtbl->Load(This, entryPoint))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_Run(This) \
    ((This)->lpVtbl->Run(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_Uninitialize(This) \
    ((This)->lpVtbl->Uninitialize(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IFrameworkViewSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IFrameworkViewSource[] = L"Windows.ApplicationModel.Core.IFrameworkViewSource";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateView)(__x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkView** viewProvider);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSourceVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_CreateView(This, viewProvider) \
    ((This)->lpVtbl->CreateView(This, viewProvider))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIFrameworkViewSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IHostedViewClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.HostedViewClosingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IHostedViewClosingEventArgs[] = L"Windows.ApplicationModel.Core.IHostedViewClosingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIHostedViewClosingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IUnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.UnhandledError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IUnhandledError[] = L"Windows.ApplicationModel.Core.IUnhandledError";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Propagate)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_Propagate(This) \
    ((This)->lpVtbl->Propagate(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Core_IUnhandledErrorDetectedEventArgs[] = L"Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnhandledError)(__x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledError** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_get_UnhandledError(This, value) \
    ((This)->lpVtbl->get_UnhandledError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCore_CIUnhandledErrorDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.AppListEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IAppListEntry ** Default Interface **
 *    Windows.ApplicationModel.Core.IAppListEntry2
 *    Windows.ApplicationModel.Core.IAppListEntry3
 *    Windows.ApplicationModel.Core.IAppListEntry4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_AppListEntry_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_AppListEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_AppListEntry[] = L"Windows.ApplicationModel.Core.AppListEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationExit interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationUnhandledError interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplication2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreImmersiveApplication interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Core.ICoreApplicationUseCount interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplication_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplication_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplication[] = L"Windows.ApplicationModel.Core.CoreApplication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.ICoreApplicationView ** Default Interface **
 *    Windows.ApplicationModel.Core.ICoreApplicationView2
 *    Windows.ApplicationModel.Core.ICoreApplicationView3
 *    Windows.ApplicationModel.Core.ICoreApplicationView5
 *    Windows.ApplicationModel.Core.ICoreApplicationView6
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationView_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplicationView[] = L"Windows.ApplicationModel.Core.CoreApplicationView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.CoreApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_CoreApplicationViewTitleBar[] = L"Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.HostedViewClosingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IHostedViewClosingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_HostedViewClosingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_HostedViewClosingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_HostedViewClosingEventArgs[] = L"Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.UnhandledError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IUnhandledError ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledError_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_UnhandledError[] = L"Windows.ApplicationModel.Core.UnhandledError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Core_UnhandledErrorDetectedEventArgs[] = L"Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
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
#endif // __windows2Eapplicationmodel2Ecore_p_h__

#endif // __windows2Eapplicationmodel2Ecore_h__
