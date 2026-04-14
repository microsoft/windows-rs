
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
#ifndef __windows2Eui2Ewebui_h__
#define __windows2Eui2Ewebui_h__
#ifndef __windows2Eui2Ewebui_p_h__
#define __windows2Eui2Ewebui_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_BACKGROUND_BACKGROUNDALARMAPPLICATIONCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.ApplicationModel.Background.h"
#include "Windows.ApplicationModel.Core.h"
#include "Windows.ApplicationModel.Wallet.h"
#include "Windows.Graphics.Printing.h"
#include "Windows.System.h"
#include "Windows.Web.UI.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IActivatedEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler ABI::Windows::UI::WebUI::IActivatedEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IBackgroundActivatedEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler ABI::Windows::UI::WebUI::IBackgroundActivatedEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IEnteredBackgroundEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler ABI::Windows::UI::WebUI::IEnteredBackgroundEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface ILeavingBackgroundEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler ABI::Windows::UI::WebUI::ILeavingBackgroundEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface INavigatedEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler ABI::Windows::UI::WebUI::INavigatedEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IResumingEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler ABI::Windows::UI::WebUI::IResumingEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface ISuspendingEventHandler;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler ABI::Windows::UI::WebUI::ISuspendingEventHandler

#endif // ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IActivatedDeferral;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral ABI::Windows::UI::WebUI::IActivatedDeferral

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IActivatedEventArgsDeferral;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral ABI::Windows::UI::WebUI::IActivatedEventArgsDeferral

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IActivatedOperation;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation ABI::Windows::UI::WebUI::IActivatedOperation

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IHtmlPrintDocumentSource;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource ABI::Windows::UI::WebUI::IHtmlPrintDocumentSource

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface INewWebUIViewCreatedEventArgs;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs ABI::Windows::UI::WebUI::INewWebUIViewCreatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIActivationStatics;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics ABI::Windows::UI::WebUI::IWebUIActivationStatics

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIActivationStatics2;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2 ABI::Windows::UI::WebUI::IWebUIActivationStatics2

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIActivationStatics3;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3 ABI::Windows::UI::WebUI::IWebUIActivationStatics3

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIActivationStatics4;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4 ABI::Windows::UI::WebUI::IWebUIActivationStatics4

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIBackgroundTaskInstance;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance ABI::Windows::UI::WebUI::IWebUIBackgroundTaskInstance

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIBackgroundTaskInstanceStatics;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics ABI::Windows::UI::WebUI::IWebUIBackgroundTaskInstanceStatics

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUINavigatedDeferral;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral ABI::Windows::UI::WebUI::IWebUINavigatedDeferral

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUINavigatedEventArgs;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs ABI::Windows::UI::WebUI::IWebUINavigatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUINavigatedOperation;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation ABI::Windows::UI::WebUI::IWebUINavigatedOperation

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIView;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView ABI::Windows::UI::WebUI::IWebUIView

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                interface IWebUIViewStatics;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics ABI::Windows::UI::WebUI::IWebUIViewStatics

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
        namespace UI {
            namespace WebUI {
                class WebUIView;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43b17969-e5df-5639-9a5c-8f18477c637e"))
IAsyncOperation<ABI::Windows::UI::WebUI::WebUIView*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::UI::WebUI::IWebUIView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.WebUI.WebUIView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::WebUI::WebUIView*> __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_t;
#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4ca655dd-3902-5030-bd60-5b4f472fd535"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::WebUI::WebUIView*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::UI::WebUI::IWebUIView*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.WebUI.WebUIView>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::WebUI::WebUIView*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                class NewWebUIViewCreatedEventArgs;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b085412-b382-5bb4-8ced-a40f247917dd"))
IEventHandler<ABI::Windows::UI::WebUI::NewWebUIViewCreatedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WebUI::NewWebUIViewCreatedEventArgs*, ABI::Windows::UI::WebUI::INewWebUIViewCreatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.UI.WebUI.NewWebUIViewCreatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::UI::WebUI::NewWebUIViewCreatedEventArgs*> __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_t;
#define __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("46fce4c3-528a-5192-a389-0d521b17cc24"))
ITypedEventHandler<ABI::Windows::UI::WebUI::WebUIView*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::UI::WebUI::IWebUIView*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WebUI.WebUIView, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WebUI::WebUIView*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8a4231a8-c4cb-5130-ba5b-b4963b42b60d"))
ITypedEventHandler<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::UI::WebUI::IWebUIView*>, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WebUI.WebUIView, Windows.ApplicationModel.Activation.IActivatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WebUI::WebUIView*, ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgsWithUser;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser ABI::Windows::ApplicationModel::Activation::IActivatedEventArgsWithUser

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IApplicationViewActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IApplicationViewActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderAddAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderRemoveAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderReplaceAppointmentActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IAppointmentsProviderShowTimeFrameActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IBarcodeScannerPreviewActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICachedFileUpdaterActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICameraSettingsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ICommandLineActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ICommandLineActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactMapActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactMapActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactMessageActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactMessageActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPanelActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPanelActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactPostActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactPostActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContactVideoCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IContinuationActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IContinuationActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDeviceActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDeviceActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDevicePairingActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDevicePairingActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IDialReceiverActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IDialReceiverActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileActivatedEventArgsWithNeighboringFiles;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles ABI::Windows::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileOpenPickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFileSavePickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IFolderPickerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IFolderPickerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILaunchActivatedEventArgs2;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2 ABI::Windows::ApplicationModel::Activation::ILaunchActivatedEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILockScreenActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILockScreenActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ILockScreenCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPhoneCallActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPhoneCallActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrelaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrelaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrint3DWorkflowActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IPrintTaskSettingsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IProtocolActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData ABI::Windows::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IProtocolForResultsActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IRestrictedLaunchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ISearchActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs ABI::Windows::ApplicationModel::Activation::ISearchActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface ISearchActivatedEventArgsWithLinguisticDetails;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails ABI::Windows::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IShareTargetActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IShareTargetActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IStartupTaskActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IStartupTaskActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IToastNotificationActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IToastNotificationActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IUserDataAccountProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IVoiceCommandActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWalletActionActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IWalletActionActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWebAccountProviderActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IWebAuthenticationBrokerContinuationEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs ABI::Windows::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTaskInstance;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance ABI::Windows::ApplicationModel::Background::IBackgroundTaskInstance

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingDeferral;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral ABI::Windows::ApplicationModel::ISuspendingDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingOperation;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation ABI::Windows::ApplicationModel::ISuspendingOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintDocumentSource;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource ABI::Windows::Graphics::Printing::IPrintDocumentSource

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControl;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl ABI::Windows::Web::UI::IWebViewControl

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControl2;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2 ABI::Windows::Web::UI::IWebViewControl2

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                typedef enum PrintContent : int PrintContent;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                class ActivatedDeferral;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                class ActivatedOperation;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                class WebUINavigatedDeferral;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                class WebUINavigatedOperation;
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.WebUI.PrintContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                enum PrintContent : int
                {
                    PrintContent_AllPages = 0,
                    PrintContent_CurrentPage = 1,
                    PrintContent_CustomPageRange = 2,
                    PrintContent_CurrentSelection = 3,
                };
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.ActivatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("50f1e730-c5d1-4b6b-9adb-8a11756be29c")
                IActivatedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs* eventArgs
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedEventHandler = __uuidof(IActivatedEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.BackgroundActivatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("edb19fbb-0761-47cc-9a77-24d7072965ca")
                IBackgroundActivatedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::ApplicationModel::Activation::IBackgroundActivatedEventArgs* eventArgs
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundActivatedEventHandler = __uuidof(IBackgroundActivatedEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Delegate Windows.UI.WebUI.EnteredBackgroundEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("2b09a173-b68e-4def-88c1-8de84e5aab2f")
                IEnteredBackgroundEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::ApplicationModel::IEnteredBackgroundEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEnteredBackgroundEventHandler = __uuidof(IEnteredBackgroundEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Delegate Windows.UI.WebUI.LeavingBackgroundEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("00b4ccd9-7a9c-4b6b-9ac4-13474f268bc4")
                ILeavingBackgroundEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::ApplicationModel::ILeavingBackgroundEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILeavingBackgroundEventHandler = __uuidof(ILeavingBackgroundEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Delegate Windows.UI.WebUI.NavigatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("7af46fe6-40ca-4e49-a7d6-dbdb330cd1a3")
                INavigatedEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::UI::WebUI::IWebUINavigatedEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INavigatedEventHandler = __uuidof(INavigatedEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.ResumingEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("26599ba9-a22d-4806-a728-acadc1d075fa")
                IResumingEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResumingEventHandler = __uuidof(IResumingEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.SuspendingEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("509c429c-78e2-4883-abc8-8960dcde1b5c")
                ISuspendingEventHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender,
                        ABI::Windows::ApplicationModel::ISuspendingEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISuspendingEventHandler = __uuidof(ISuspendingEventHandler);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.ActivatedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedDeferral[] = L"Windows.UI.WebUI.IActivatedDeferral";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("c3bd1978-a431-49d8-a76a-395a4e03dcf3")
                IActivatedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedDeferral = __uuidof(IActivatedDeferral);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedEventArgsDeferral[] = L"Windows.UI.WebUI.IActivatedEventArgsDeferral";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("ca6d5f74-63c2-44a6-b97b-d9a03c20bc9b")
                IActivatedEventArgsDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatedOperation(
                        ABI::Windows::UI::WebUI::IActivatedOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedEventArgsDeferral = __uuidof(IActivatedEventArgsDeferral);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.ActivatedOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedOperation[] = L"Windows.UI.WebUI.IActivatedOperation";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("b6a0b4bc-c6ca-42fd-9818-71904e45fed7")
                IActivatedOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::UI::WebUI::IActivatedDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivatedOperation = __uuidof(IActivatedOperation);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IHtmlPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.HtmlPrintDocumentSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.IPrintDocumentSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IHtmlPrintDocumentSource[] = L"Windows.UI.WebUI.IHtmlPrintDocumentSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("cea6469a-0e05-467a-abc9-36ec1d4cdcb6")
                IHtmlPrintDocumentSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::UI::WebUI::PrintContent* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Content(
                        ABI::Windows::UI::WebUI::PrintContent value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LeftMargin(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LeftMargin(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TopMargin(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TopMargin(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RightMargin(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RightMargin(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BottomMargin(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BottomMargin(
                        FLOAT value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EnableHeaderFooter(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EnableHeaderFooter(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShrinkToFit(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShrinkToFit(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PercentScale(
                        FLOAT* pScalePercent
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PercentScale(
                        FLOAT scalePercent
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PageRange(
                        HSTRING* pstrPageRange
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetPageRange(
                        HSTRING strPageRange,
                        boolean* pfSuccess
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHtmlPrintDocumentSource = __uuidof(IHtmlPrintDocumentSource);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.INewWebUIViewCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.NewWebUIViewCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_INewWebUIViewCreatedEventArgs[] = L"Windows.UI.WebUI.INewWebUIViewCreatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("e8e1b216-be2b-4c9e-85e7-083143ec4be7")
                INewWebUIViewCreatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebUIView(
                        ABI::Windows::UI::WebUI::IWebUIView** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatedEventArgs(
                        ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasPendingNavigate(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INewWebUIViewCreatedEventArgs = __uuidof(INewWebUIViewCreatedEventArgs);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics[] = L"Windows.UI.WebUI.IWebUIActivationStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("351b86bd-43b3-482b-85db-35d87b517ad9")
                IWebUIActivationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Activated(
                        ABI::Windows::UI::WebUI::IActivatedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Suspending(
                        ABI::Windows::UI::WebUI::ISuspendingEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Suspending(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Resuming(
                        ABI::Windows::UI::WebUI::IResumingEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Resuming(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Navigated(
                        ABI::Windows::UI::WebUI::INavigatedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Navigated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIActivationStatics = __uuidof(IWebUIActivationStatics);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics2[] = L"Windows.UI.WebUI.IWebUIActivationStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("c8e88696-4d78-4aa4-8f06-2a9eadc6c40a")
                IWebUIActivationStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_LeavingBackground(
                        ABI::Windows::UI::WebUI::ILeavingBackgroundEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LeavingBackground(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnteredBackground(
                        ABI::Windows::UI::WebUI::IEnteredBackgroundEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnteredBackground(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnablePrelaunch(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIActivationStatics2 = __uuidof(IWebUIActivationStatics2);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics3[] = L"Windows.UI.WebUI.IWebUIActivationStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("91abb686-1af5-4445-b49f-9459f40fc8de")
                IWebUIActivationStatics3 : public IInspectable
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

                MIDL_CONST_ID IID& IID_IWebUIActivationStatics3 = __uuidof(IWebUIActivationStatics3);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics4[] = L"Windows.UI.WebUI.IWebUIActivationStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("5e391429-183f-478d-8a25-67f80d03935b")
                IWebUIActivationStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_NewWebUIViewCreated(
                        __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NewWebUIViewCreated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_BackgroundActivated(
                        ABI::Windows::UI::WebUI::IBackgroundActivatedEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BackgroundActivated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIActivationStatics4 = __uuidof(IWebUIActivationStatics4);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIBackgroundTaskInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIBackgroundTaskInstance[] = L"Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("23f12c25-e2f7-4741-bc9c-394595de24dc")
                IWebUIBackgroundTaskInstance : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* succeeded
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Succeeded(
                        boolean succeeded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIBackgroundTaskInstance = __uuidof(IWebUIBackgroundTaskInstance);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIBackgroundTaskInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIBackgroundTaskInstanceStatics[] = L"Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("9c7a5291-19ae-4ca3-b94b-fe4ec744a740")
                IWebUIBackgroundTaskInstanceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Current(
                        ABI::Windows::UI::WebUI::IWebUIBackgroundTaskInstance** backgroundTaskInstance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIBackgroundTaskInstanceStatics = __uuidof(IWebUIBackgroundTaskInstanceStatics);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUINavigatedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedDeferral[] = L"Windows.UI.WebUI.IWebUINavigatedDeferral";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("d804204d-831f-46e2-b432-3afce211f962")
                IWebUINavigatedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUINavigatedDeferral = __uuidof(IWebUINavigatedDeferral);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedEventArgs[] = L"Windows.UI.WebUI.IWebUINavigatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("a75841b8-2499-4030-a69d-15d2d9cfe524")
                IWebUINavigatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NavigatedOperation(
                        ABI::Windows::UI::WebUI::IWebUINavigatedOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUINavigatedEventArgs = __uuidof(IWebUINavigatedEventArgs);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUINavigatedOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedOperation[] = L"Windows.UI.WebUI.IWebUINavigatedOperation";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("7a965f08-8182-4a89-ab67-8492e8750d4b")
                IWebUINavigatedOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::UI::WebUI::IWebUINavigatedDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUINavigatedOperation = __uuidof(IWebUINavigatedOperation);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIView[] = L"Windows.UI.WebUI.IWebUIView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("6783f64f-52da-4fd7-be69-8ef6284b423c")
                IWebUIView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationViewId(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Activated(
                        __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IgnoreApplicationContentUriRulesNavigationRestrictions(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IgnoreApplicationContentUriRulesNavigationRestrictions(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIView = __uuidof(IWebUIView);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIView;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIViewStatics[] = L"Windows.UI.WebUI.IWebUIViewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WebUI {
                MIDL_INTERFACE("b591e668-8e59-44f9-8803-1b24c9149d30")
                IWebUIViewStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithUriAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebUIViewStatics = __uuidof(IWebUIViewStatics);
            } /* WebUI */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.ActivatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IActivatedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_ActivatedDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_ActivatedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_ActivatedDeferral[] = L"Windows.UI.WebUI.ActivatedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.ActivatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IActivatedOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_ActivatedOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_ActivatedOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_ActivatedOperation[] = L"Windows.UI.WebUI.ActivatedOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.BackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_BackgroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_BackgroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_BackgroundActivatedEventArgs[] = L"Windows.UI.WebUI.BackgroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.EnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IEnteredBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_EnteredBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_EnteredBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_EnteredBackgroundEventArgs[] = L"Windows.UI.WebUI.EnteredBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.HtmlPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IHtmlPrintDocumentSource ** Default Interface **
 *    Windows.Graphics.Printing.IPrintDocumentSource
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_HtmlPrintDocumentSource_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_HtmlPrintDocumentSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_HtmlPrintDocumentSource[] = L"Windows.UI.WebUI.HtmlPrintDocumentSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.LeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILeavingBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_LeavingBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_LeavingBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_LeavingBackgroundEventArgs[] = L"Windows.UI.WebUI.LeavingBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.NewWebUIViewCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.INewWebUIViewCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs[] = L"Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.SuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingDeferral[] = L"Windows.UI.WebUI.SuspendingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.SuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingEventArgs[] = L"Windows.UI.WebUI.SuspendingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.SuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingOperation[] = L"Windows.UI.WebUI.SuspendingOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics4 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIApplication_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIApplication_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIApplication[] = L"Windows.UI.WebUI.WebUIApplication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBackgroundTaskInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstance_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBackgroundTaskInstance[] = L"Windows.UI.WebUI.WebUIBackgroundTaskInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUIBackgroundTaskInstance ** Default Interface **
 *    Windows.ApplicationModel.Background.IBackgroundTaskInstance
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass[] = L"Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUICommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIDeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILaunchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedDeferral[] = L"Windows.UI.WebUI.WebUINavigatedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedEventArgs[] = L"Windows.UI.WebUI.WebUINavigatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedOperation[] = L"Windows.UI.WebUI.WebUINavigatedOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUIProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUISearchActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.WebUISearchActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUISearchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUISearchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUISearchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUISearchActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.WebUIView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIViewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUIView ** Default Interface **
 *    Windows.Web.UI.IWebViewControl
 *    Windows.Web.UI.IWebViewControl2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIView_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIView[] = L"Windows.UI.WebUI.WebUIView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("WebUIWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2 __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3 __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4 __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIView __x_ABI_CWindows_CUI_CWebUI_CIWebUIView;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics;

#endif // ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView;

typedef struct __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUIView** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIViewVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView* This,
        __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIViewVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWebUI__CWebUIView_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs;

typedef struct __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUIView* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUIView* sender,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIApplicationViewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderAddAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderRemoveAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderReplaceAppointmentActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIAppointmentsProviderShowTimeFrameActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIBarcodeScannerPreviewActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICachedFileUpdaterActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICameraSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CICommandLineActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMapActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactMessageActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPanelActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactPostActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContactVideoCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIContinuationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDeviceActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDevicePairingActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIDialReceiverActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles __x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileActivatedEventArgsWithNeighboringFiles_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileOpenPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFileSavePickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIFolderPickerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2 __x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILaunchActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CILockScreenCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPhoneCallActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrelaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrint3DWorkflowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIPrintTaskSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIProtocolForResultsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIRestrictedLaunchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails __x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CISearchActivatedEventArgsWithLinguisticDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIShareTargetActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIStartupTaskActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIToastNotificationActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIUserDataAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIVoiceCommandActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWalletActionActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAccountProviderActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIWebAuthenticationBrokerContinuationEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTaskInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingOperation __x_ABI_CWindows_CApplicationModel_CISuspendingOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl __x_ABI_CWindows_CWeb_CUI_CIWebViewControl;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2 __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CWebUI_CPrintContent __x_ABI_CWindows_CUI_CWebUI_CPrintContent;

/*
 *
 * Struct Windows.UI.WebUI.PrintContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CWebUI_CPrintContent
{
    PrintContent_AllPages = 0,
    PrintContent_CurrentPage = 1,
    PrintContent_CustomPageRange = 2,
    PrintContent_CurrentSelection = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.ActivatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs* eventArgs);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_Invoke(This, sender, eventArgs) \
    ((This)->lpVtbl->Invoke(This, sender, eventArgs))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.BackgroundActivatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIBackgroundActivatedEventArgs* eventArgs);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_Invoke(This, sender, eventArgs) \
    ((This)->lpVtbl->Invoke(This, sender, eventArgs))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Delegate Windows.UI.WebUI.EnteredBackgroundEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Delegate Windows.UI.WebUI.LeavingBackgroundEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Delegate Windows.UI.WebUI.NavigatedEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.ResumingEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler* This,
        IInspectable* sender);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.WebUI.SuspendingEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.ActivatedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedDeferral[] = L"Windows.UI.WebUI.IActivatedDeferral";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferralVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedEventArgsDeferral[] = L"Windows.UI.WebUI.IActivatedEventArgsDeferral";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivatedOperation)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral* This,
        __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferralVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_get_ActivatedOperation(This, value) \
    ((This)->lpVtbl->get_ActivatedOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedEventArgsDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IActivatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.ActivatedOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IActivatedOperation[] = L"Windows.UI.WebUI.IActivatedOperation";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation* This,
        __x_ABI_CWindows_CUI_CWebUI_CIActivatedDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperationVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIActivatedOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IHtmlPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.HtmlPrintDocumentSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.IPrintDocumentSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IHtmlPrintDocumentSource[] = L"Windows.UI.WebUI.IHtmlPrintDocumentSource";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        enum __x_ABI_CWindows_CUI_CWebUI_CPrintContent* value);
    HRESULT (STDMETHODCALLTYPE* put_Content)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        enum __x_ABI_CWindows_CUI_CWebUI_CPrintContent value);
    HRESULT (STDMETHODCALLTYPE* get_LeftMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_LeftMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_TopMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_TopMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_RightMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_RightMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_BottomMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_BottomMargin)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_EnableHeaderFooter)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_EnableHeaderFooter)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShrinkToFit)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShrinkToFit)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PercentScale)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT* pScalePercent);
    HRESULT (STDMETHODCALLTYPE* put_PercentScale)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        FLOAT scalePercent);
    HRESULT (STDMETHODCALLTYPE* get_PageRange)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        HSTRING* pstrPageRange);
    HRESULT (STDMETHODCALLTYPE* TrySetPageRange)(__x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource* This,
        HSTRING strPageRange,
        boolean* pfSuccess);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSourceVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_Content(This, value) \
    ((This)->lpVtbl->put_Content(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_LeftMargin(This, value) \
    ((This)->lpVtbl->get_LeftMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_LeftMargin(This, value) \
    ((This)->lpVtbl->put_LeftMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_TopMargin(This, value) \
    ((This)->lpVtbl->get_TopMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_TopMargin(This, value) \
    ((This)->lpVtbl->put_TopMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_RightMargin(This, value) \
    ((This)->lpVtbl->get_RightMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_RightMargin(This, value) \
    ((This)->lpVtbl->put_RightMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_BottomMargin(This, value) \
    ((This)->lpVtbl->get_BottomMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_BottomMargin(This, value) \
    ((This)->lpVtbl->put_BottomMargin(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_EnableHeaderFooter(This, value) \
    ((This)->lpVtbl->get_EnableHeaderFooter(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_EnableHeaderFooter(This, value) \
    ((This)->lpVtbl->put_EnableHeaderFooter(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_ShrinkToFit(This, value) \
    ((This)->lpVtbl->get_ShrinkToFit(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_ShrinkToFit(This, value) \
    ((This)->lpVtbl->put_ShrinkToFit(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_PercentScale(This, pScalePercent) \
    ((This)->lpVtbl->get_PercentScale(This, pScalePercent))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_put_PercentScale(This, scalePercent) \
    ((This)->lpVtbl->put_PercentScale(This, scalePercent))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_get_PageRange(This, pstrPageRange) \
    ((This)->lpVtbl->get_PageRange(This, pstrPageRange))

#define __x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_TrySetPageRange(This, strPageRange, pfSuccess) \
    ((This)->lpVtbl->TrySetPageRange(This, strPageRange, pfSuccess))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIHtmlPrintDocumentSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.INewWebUIViewCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.NewWebUIViewCreatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_INewWebUIViewCreatedEventArgs[] = L"Windows.UI.WebUI.INewWebUIViewCreatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebUIView)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUIView** value);
    HRESULT (STDMETHODCALLTYPE* get_ActivatedEventArgs)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs** value);
    HRESULT (STDMETHODCALLTYPE* get_HasPendingNavigate)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_get_WebUIView(This, value) \
    ((This)->lpVtbl->get_WebUIView(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_get_ActivatedEventArgs(This, value) \
    ((This)->lpVtbl->get_ActivatedEventArgs(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_get_HasPendingNavigate(This, value) \
    ((This)->lpVtbl->get_HasPendingNavigate(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CINewWebUIViewCreatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics[] = L"Windows.UI.WebUI.IWebUIActivationStatics";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        __x_ABI_CWindows_CUI_CWebUI_CIActivatedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Suspending)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        __x_ABI_CWindows_CUI_CWebUI_CISuspendingEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Suspending)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Resuming)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        __x_ABI_CWindows_CUI_CWebUI_CIResumingEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Resuming)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Navigated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        __x_ABI_CWindows_CUI_CWebUI_CINavigatedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Navigated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_add_Activated(This, handler, token) \
    ((This)->lpVtbl->add_Activated(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_remove_Activated(This, token) \
    ((This)->lpVtbl->remove_Activated(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_add_Suspending(This, handler, token) \
    ((This)->lpVtbl->add_Suspending(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_remove_Suspending(This, token) \
    ((This)->lpVtbl->remove_Suspending(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_add_Resuming(This, handler, token) \
    ((This)->lpVtbl->add_Resuming(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_remove_Resuming(This, token) \
    ((This)->lpVtbl->remove_Resuming(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_add_Navigated(This, handler, token) \
    ((This)->lpVtbl->add_Navigated(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_remove_Navigated(This, token) \
    ((This)->lpVtbl->remove_Navigated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics2[] = L"Windows.UI.WebUI.IWebUIActivationStatics2";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_LeavingBackground)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        __x_ABI_CWindows_CUI_CWebUI_CILeavingBackgroundEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LeavingBackground)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnteredBackground)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        __x_ABI_CWindows_CUI_CWebUI_CIEnteredBackgroundEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnteredBackground)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* EnablePrelaunch)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_add_LeavingBackground(This, handler, token) \
    ((This)->lpVtbl->add_LeavingBackground(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_remove_LeavingBackground(This, token) \
    ((This)->lpVtbl->remove_LeavingBackground(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_add_EnteredBackground(This, handler, token) \
    ((This)->lpVtbl->add_EnteredBackground(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_remove_EnteredBackground(This, token) \
    ((This)->lpVtbl->remove_EnteredBackground(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_EnablePrelaunch(This, value) \
    ((This)->lpVtbl->EnablePrelaunch(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics3[] = L"Windows.UI.WebUI.IWebUIActivationStatics3";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestRestartAsync)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        HSTRING launchArguments,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation);
    HRESULT (STDMETHODCALLTYPE* RequestRestartForUserAsync)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING launchArguments,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCore__CAppRestartFailureReason** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_RequestRestartAsync(This, launchArguments, operation) \
    ((This)->lpVtbl->RequestRestartAsync(This, launchArguments, operation))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_RequestRestartForUserAsync(This, user, launchArguments, operation) \
    ((This)->lpVtbl->RequestRestartForUserAsync(This, user, launchArguments, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIActivationStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIApplication
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIActivationStatics4[] = L"Windows.UI.WebUI.IWebUIActivationStatics4";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_NewWebUIViewCreated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        __FIEventHandler_1_Windows__CUI__CWebUI__CNewWebUIViewCreatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NewWebUIViewCreated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_BackgroundActivated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        __x_ABI_CWindows_CUI_CWebUI_CIBackgroundActivatedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BackgroundActivated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_add_NewWebUIViewCreated(This, handler, token) \
    ((This)->lpVtbl->add_NewWebUIViewCreated(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_remove_NewWebUIViewCreated(This, token) \
    ((This)->lpVtbl->remove_NewWebUIViewCreated(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_add_BackgroundActivated(This, handler, token) \
    ((This)->lpVtbl->add_BackgroundActivated(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_remove_BackgroundActivated(This, token) \
    ((This)->lpVtbl->remove_BackgroundActivated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIActivationStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIBackgroundTaskInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIBackgroundTaskInstance[] = L"Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        boolean* succeeded);
    HRESULT (STDMETHODCALLTYPE* put_Succeeded)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance* This,
        boolean succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_get_Succeeded(This, succeeded) \
    ((This)->lpVtbl->get_Succeeded(This, succeeded))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_put_Succeeded(This, succeeded) \
    ((This)->lpVtbl->put_Succeeded(This, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIBackgroundTaskInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIBackgroundTaskInstanceStatics[] = L"Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstance** backgroundTaskInstance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_get_Current(This, backgroundTaskInstance) \
    ((This)->lpVtbl->get_Current(This, backgroundTaskInstance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIBackgroundTaskInstanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUINavigatedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedDeferral[] = L"Windows.UI.WebUI.IWebUINavigatedDeferral";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferralVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedEventArgs[] = L"Windows.UI.WebUI.IWebUINavigatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NavigatedOperation)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_get_NavigatedOperation(This, value) \
    ((This)->lpVtbl->get_NavigatedOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUINavigatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUINavigatedOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUINavigatedOperation[] = L"Windows.UI.WebUI.IWebUINavigatedOperation";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation* This,
        __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperationVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUINavigatedOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIView[] = L"Windows.UI.WebUI.IWebUIView";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationViewId)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        __FITypedEventHandler_2_Windows__CUI__CWebUI__CWebUIView_Windows__CApplicationModel__CActivation__CIActivatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IgnoreApplicationContentUriRulesNavigationRestrictions)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IgnoreApplicationContentUriRulesNavigationRestrictions)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIView* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_get_ApplicationViewId(This, value) \
    ((This)->lpVtbl->get_ApplicationViewId(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_add_Closed(This, handler, token) \
    ((This)->lpVtbl->add_Closed(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_add_Activated(This, handler, token) \
    ((This)->lpVtbl->add_Activated(This, handler, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_remove_Activated(This, token) \
    ((This)->lpVtbl->remove_Activated(This, token))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_get_IgnoreApplicationContentUriRulesNavigationRestrictions(This, value) \
    ((This)->lpVtbl->get_IgnoreApplicationContentUriRulesNavigationRestrictions(This, value))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIView_put_IgnoreApplicationContentUriRulesNavigationRestrictions(This, value) \
    ((This)->lpVtbl->put_IgnoreApplicationContentUriRulesNavigationRestrictions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIView;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.WebUI.IWebUIViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.WebUI.WebUIView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WebUI_IWebUIViewStatics[] = L"Windows.UI.WebUI.IWebUIViewStatics";
typedef struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView** operation);
    HRESULT (STDMETHODCALLTYPE* CreateWithUriAsync)(__x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CUI__CWebUI__CWebUIView** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_CreateAsync(This, operation) \
    ((This)->lpVtbl->CreateAsync(This, operation))

#define __x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_CreateWithUriAsync(This, uri, operation) \
    ((This)->lpVtbl->CreateWithUriAsync(This, uri, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWebUI_CIWebUIViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.ActivatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IActivatedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_ActivatedDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_ActivatedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_ActivatedDeferral[] = L"Windows.UI.WebUI.ActivatedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.ActivatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IActivatedOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_ActivatedOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_ActivatedOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_ActivatedOperation[] = L"Windows.UI.WebUI.ActivatedOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.BackgroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBackgroundActivatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_BackgroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_BackgroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_BackgroundActivatedEventArgs[] = L"Windows.UI.WebUI.BackgroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.EnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IEnteredBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_EnteredBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_EnteredBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_EnteredBackgroundEventArgs[] = L"Windows.UI.WebUI.EnteredBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.HtmlPrintDocumentSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IHtmlPrintDocumentSource ** Default Interface **
 *    Windows.Graphics.Printing.IPrintDocumentSource
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_HtmlPrintDocumentSource_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_HtmlPrintDocumentSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_HtmlPrintDocumentSource[] = L"Windows.UI.WebUI.HtmlPrintDocumentSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.LeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILeavingBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_LeavingBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_LeavingBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_LeavingBackgroundEventArgs[] = L"Windows.UI.WebUI.LeavingBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.NewWebUIViewCreatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.INewWebUIViewCreatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_NewWebUIViewCreatedEventArgs[] = L"Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.SuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingDeferral[] = L"Windows.UI.WebUI.SuspendingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.SuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingEventArgs[] = L"Windows.UI.WebUI.SuspendingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.SuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_SuspendingOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_SuspendingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_SuspendingOperation[] = L"Windows.UI.WebUI.SuspendingOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIApplication
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics3 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics4 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIActivationStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIApplication_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIApplication_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIApplication[] = L"Windows.UI.WebUI.WebUIApplication";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderAddAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderAddAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderRemoveAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderReplaceAppointmentActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderShowTimeFrameActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IAppointmentsProviderActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBackgroundTaskInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstance_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBackgroundTaskInstance[] = L"Windows.UI.WebUI.WebUIBackgroundTaskInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUIBackgroundTaskInstance ** Default Interface **
 *    Windows.ApplicationModel.Background.IBackgroundTaskInstance
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBackgroundTaskInstanceRuntimeClass[] = L"Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IBarcodeScannerPreviewActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIBarcodeScannerPreviewActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICachedFileUpdaterActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICachedFileUpdaterActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivationCameraSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICameraSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICameraSettingsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUICommandLineActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ICommandLineActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUICommandLineActivatedEventArgs[] = L"Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactMapActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMapActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactMapActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactMessageActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactMessageActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPanelActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPanelActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactPostActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactPostActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactPostActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ContactActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IContactVideoCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContactActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIContactVideoCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIDeviceActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDeviceActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDeviceActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDevicePairingActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDevicePairingActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IDialReceiverActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIDialReceiverActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IFileActivatedEventArgsWithNeighboringFiles
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileOpenPickerActivatedEventArgs2
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileOpenPickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileOpenPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFileOpenPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileOpenPickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IFileSavePickerActivatedEventArgs2
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileSavePickerActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFileSavePickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFileSavePickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFileSavePickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IFolderPickerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("WebUIFolderPickerContinuationEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIFolderPickerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILaunchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ILockScreenCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUILockScreenComponentActivatedEventArgs[] = L"Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedDeferral[] = L"Windows.UI.WebUI.WebUINavigatedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedEventArgs[] = L"Windows.UI.WebUI.WebUINavigatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUINavigatedOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUINavigatedOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedOperation_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUINavigatedOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUINavigatedOperation[] = L"Windows.UI.WebUI.WebUINavigatedOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPhoneCallActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPhoneCallActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrint3DWorkflowActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrint3DWorkflowActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.ActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IPrintTaskSettingsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrintTaskSettingsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs ** Default Interface **
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIPrintWorkflowForegroundTaskActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.WebUI.WebUIProtocolActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIProtocolActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IProtocolForResultsActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIProtocolForResultsActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IRestrictedLaunchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIRestrictedLaunchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUISearchActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Activation.WebUISearchActivatedEventsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.ISearchActivatedEventArgsWithLinguisticDetails
 *    Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUISearchActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUISearchActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUISearchActivatedEventArgs[] = L"Windows.UI.WebUI.WebUISearchActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IShareTargetActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIShareTargetActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IStartupTaskActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIStartupTaskActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IToastNotificationActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIToastNotificationActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IUserDataAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIUserDataAccountProviderActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.WebUI.WebUIView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WebUI.IWebUIViewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WebUI.IWebUIView ** Default Interface **
 *    Windows.Web.UI.IWebViewControl
 *    Windows.Web.UI.IWebViewControl2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIView_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIView[] = L"Windows.UI.WebUI.WebUIView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IVoiceCommandActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIVoiceCommandActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWalletActionActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs_DEFINED
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("WebUIWalletActionActivatedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWalletActionActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAccountProviderActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWebAccountProviderActivatedEventArgs[] = L"Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Activation.IWebAuthenticationBrokerContinuationEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IContinuationActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.UI.WebUI.IActivatedEventArgsDeferral
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WebUI_WebUIWebAuthenticationBrokerContinuationEventArgs[] = L"Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
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
#endif // __windows2Eui2Ewebui_p_h__

#endif // __windows2Eui2Ewebui_h__
