
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
#ifndef __windows2Eweb2Eui_h__
#define __windows2Eweb2Eui_h__
#ifndef __windows2Eweb2Eui_p_h__
#define __windows2Eweb2Eui_p_h__


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
#include "Windows.ApplicationModel.DataTransfer.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
#include "Windows.Web.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlContentLoadingEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs ABI::Windows::Web::UI::IWebViewControlContentLoadingEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlDOMContentLoadedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs ABI::Windows::Web::UI::IWebViewControlDOMContentLoadedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlDeferredPermissionRequest;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest ABI::Windows::Web::UI::IWebViewControlDeferredPermissionRequest

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlLongRunningScriptDetectedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs ABI::Windows::Web::UI::IWebViewControlLongRunningScriptDetectedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlNavigationCompletedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs ABI::Windows::Web::UI::IWebViewControlNavigationCompletedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlNavigationStartingEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs ABI::Windows::Web::UI::IWebViewControlNavigationStartingEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlNewWindowRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs ABI::Windows::Web::UI::IWebViewControlNewWindowRequestedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlNewWindowRequestedEventArgs2;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2 ABI::Windows::Web::UI::IWebViewControlNewWindowRequestedEventArgs2

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlPermissionRequest;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest ABI::Windows::Web::UI::IWebViewControlPermissionRequest

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlPermissionRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs ABI::Windows::Web::UI::IWebViewControlPermissionRequestedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlScriptNotifyEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs ABI::Windows::Web::UI::IWebViewControlScriptNotifyEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlSettings;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings ABI::Windows::Web::UI::IWebViewControlSettings

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs ABI::Windows::Web::UI::IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlUnviewableContentIdentifiedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs ABI::Windows::Web::UI::IWebViewControlUnviewableContentIdentifiedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                interface IWebViewControlWebResourceRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs ABI::Windows::Web::UI::IWebViewControlWebResourceRequestedEventArgs

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a16f2d07-ead3-53e4-9490-75bdbaeb7a5b"))
IAsyncOperation<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::IDataPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.DataTransfer.DataPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*> __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a93a3b99-e946-57ce-aad9-c23d138c353e"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*, ABI::Windows::ApplicationModel::DataTransfer::IDataPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.DataTransfer.DataPackage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::DataTransfer::DataPackage*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_USE */

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


namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlDeferredPermissionRequest;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#define DEF___FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("277a80bd-3e2a-5fe8-95d2-cc86f723bf42"))
IIterator<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*, ABI::Windows::Web::UI::IWebViewControlDeferredPermissionRequest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.UI.WebViewControlDeferredPermissionRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t;
#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#define DEF___FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("91805f3f-13cb-5483-b592-b9ae0e334f17"))
IIterable<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*, ABI::Windows::Web::UI::IWebViewControlDeferredPermissionRequest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.UI.WebViewControlDeferredPermissionRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t;
#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#define DEF___FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bffd3dcf-1974-53a2-8d88-966d84ba98e0"))
IVectorView<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*, ABI::Windows::Web::UI::IWebViewControlDeferredPermissionRequest*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.UI.WebViewControlDeferredPermissionRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Web::UI::WebViewControlDeferredPermissionRequest*> __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t;
#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b5089479-abd8-5985-8a93-4c208a85e3a4"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, IInspectable*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlContentLoadingEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("357154fe-ec1a-5fca-b860-62f03dece49d"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlContentLoadingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlContentLoadingEventArgs*, ABI::Windows::Web::UI::IWebViewControlContentLoadingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlContentLoadingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlContentLoadingEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlDOMContentLoadedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d7dc333e-b521-5b76-a7ff-48b454f597e5"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlDOMContentLoadedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlDOMContentLoadedEventArgs*, ABI::Windows::Web::UI::IWebViewControlDOMContentLoadedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlDOMContentLoadedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlLongRunningScriptDetectedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd29249e-4112-533d-96a2-0a1c01519caf"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs*, ABI::Windows::Web::UI::IWebViewControlLongRunningScriptDetectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlNavigationCompletedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("57a87c53-47a5-5864-9881-fd4c00f230a9"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNavigationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlNavigationCompletedEventArgs*, ABI::Windows::Web::UI::IWebViewControlNavigationCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlNavigationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNavigationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlNavigationStartingEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e92e0bcc-9ae9-5b9b-a684-83dd8ee57775"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNavigationStartingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlNavigationStartingEventArgs*, ABI::Windows::Web::UI::IWebViewControlNavigationStartingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlNavigationStartingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNavigationStartingEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlNewWindowRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("317111df-10c6-559c-85a1-847eb0a1b2d5"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNewWindowRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlNewWindowRequestedEventArgs*, ABI::Windows::Web::UI::IWebViewControlNewWindowRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlNewWindowRequestedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlPermissionRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e5cacc26-2a68-5e0a-b82b-b3d756e10a56"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlPermissionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlPermissionRequestedEventArgs*, ABI::Windows::Web::UI::IWebViewControlPermissionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlPermissionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlPermissionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlScriptNotifyEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee8b81d3-bbc2-55b0-877b-6ba86e3ad899"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlScriptNotifyEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlScriptNotifyEventArgs*, ABI::Windows::Web::UI::IWebViewControlScriptNotifyEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlScriptNotifyEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlScriptNotifyEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlUnsupportedUriSchemeIdentifiedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8658050c-5e47-5516-b25b-57fae22c4b88"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs*, ABI::Windows::Web::UI::IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlUnviewableContentIdentifiedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("571626e3-9780-5b37-be8a-ab8e4e7898cf"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs*, ABI::Windows::Web::UI::IWebViewControlUnviewableContentIdentifiedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlWebResourceRequestedEventArgs;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3a6ed2bc-032b-5ec7-a20a-c1ef49250c3c"))
ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlWebResourceRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Web::UI::WebViewControlWebResourceRequestedEventArgs*, ABI::Windows::Web::UI::IWebViewControlWebResourceRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Web.UI.IWebViewControl, Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Web::UI::IWebViewControl*, ABI::Windows::Web::UI::WebViewControlWebResourceRequestedEventArgs*> __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpRequestMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage ABI::Windows::Web::Http::IHttpRequestMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            interface IUriToStreamResolver;
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CIUriToStreamResolver ABI::Windows::Web::IUriToStreamResolver

#endif // ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            typedef enum WebErrorStatus : int WebErrorStatus;
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                typedef enum WebViewControlPermissionState : int WebViewControlPermissionState;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                typedef enum WebViewControlPermissionType : int WebViewControlPermissionType;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlPermissionRequest;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                class WebViewControlSettings;
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Web.UI.WebViewControlPermissionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                enum WebViewControlPermissionState : int
                {
                    WebViewControlPermissionState_Unknown = 0,
                    WebViewControlPermissionState_Defer = 1,
                    WebViewControlPermissionState_Allow = 2,
                    WebViewControlPermissionState_Deny = 3,
                };
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Web.UI.WebViewControlPermissionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                enum WebViewControlPermissionType : int
                {
                    WebViewControlPermissionType_Geolocation = 0,
                    WebViewControlPermissionType_UnlimitedIndexedDBQuota = 1,
                    WebViewControlPermissionType_Media = 2,
                    WebViewControlPermissionType_PointerLock = 3,
                    WebViewControlPermissionType_WebNotifications = 4,
                    WebViewControlPermissionType_Screen = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    WebViewControlPermissionType_ImmersiveView = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                };
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControl[] = L"Windows.Web.UI.IWebViewControl";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("3f921316-bc70-4bda-9136-c94370899fab")
                IWebViewControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Source(
                        ABI::Windows::Foundation::IUriRuntimeClass* source
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentTitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanGoBack(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanGoForward(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultBackgroundColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultBackgroundColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContainsFullScreenElement(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Settings(
                        ABI::Windows::Web::UI::IWebViewControlSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeferredPermissionRequests(
                        __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GoForward(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GoBack(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Refresh(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Navigate(
                        ABI::Windows::Foundation::IUriRuntimeClass* source
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NavigateToString(
                        HSTRING text
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NavigateToLocalStreamUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* source,
                        ABI::Windows::Web::IUriToStreamResolver* streamResolver
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NavigateWithHttpRequestMessage(
                        ABI::Windows::Web::Http::IHttpRequestMessage* requestMessage
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE InvokeScriptAsync(
                        HSTRING scriptName,
                        __FIIterable_1_HSTRING* arguments,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CapturePreviewToStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CaptureSelectedContentToDataPackageAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE BuildLocalStreamUri(
                        HSTRING contentIdentifier,
                        HSTRING relativePath,
                        ABI::Windows::Foundation::IUriRuntimeClass** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferredPermissionRequestById(
                        UINT32 id,
                        ABI::Windows::Web::UI::IWebViewControlDeferredPermissionRequest** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NavigationStarting(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NavigationStarting(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ContentLoading(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ContentLoading(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DOMContentLoaded(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DOMContentLoaded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NavigationCompleted(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NavigationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FrameNavigationStarting(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FrameNavigationStarting(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FrameContentLoading(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FrameContentLoading(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FrameDOMContentLoaded(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FrameDOMContentLoaded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FrameNavigationCompleted(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FrameNavigationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScriptNotify(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScriptNotify(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LongRunningScriptDetected(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LongRunningScriptDetected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UnsafeContentWarningDisplaying(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UnsafeContentWarningDisplaying(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UnviewableContentIdentified(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UnviewableContentIdentified(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PermissionRequested(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PermissionRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UnsupportedUriSchemeIdentified(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UnsupportedUriSchemeIdentified(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NewWindowRequested(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NewWindowRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ContainsFullScreenElementChanged(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ContainsFullScreenElementChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_WebResourceRequested(
                        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_WebResourceRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControl = __uuidof(IWebViewControl);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControl;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControl2[] = L"Windows.Web.UI.IWebViewControl2";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("4d3c06f9-c8df-41cc-8bd5-2a947b204503")
                IWebViewControl2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddInitializeScript(
                        HSTRING script
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControl2 = __uuidof(IWebViewControl2);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControl2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlContentLoadingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlContentLoadingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlContentLoadingEventArgs[] = L"Windows.Web.UI.IWebViewControlContentLoadingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("9a3fccb2-b9bb-404b-a22b-66dccd1250c6")
                IWebViewControlContentLoadingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlContentLoadingEventArgs = __uuidof(IWebViewControlContentLoadingEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlDOMContentLoadedEventArgs[] = L"Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("be8bc008-9541-4545-9ff2-2df585b29f7d")
                IWebViewControlDOMContentLoadedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlDOMContentLoadedEventArgs = __uuidof(IWebViewControlDOMContentLoadedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlDeferredPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlDeferredPermissionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlDeferredPermissionRequest[] = L"Windows.Web.UI.IWebViewControlDeferredPermissionRequest";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("2ce349e0-d759-445c-9926-8995298f152b")
                IWebViewControlDeferredPermissionRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PermissionType(
                        ABI::Windows::Web::UI::WebViewControlPermissionType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Allow(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Deny(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlDeferredPermissionRequest = __uuidof(IWebViewControlDeferredPermissionRequest);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlLongRunningScriptDetectedEventArgs[] = L"Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("2a6e5bba-98b4-45bc-bbeb-0f69ce49c599")
                IWebViewControlLongRunningScriptDetectedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutionTime(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StopPageScriptExecution(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StopPageScriptExecution(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlLongRunningScriptDetectedEventArgs = __uuidof(IWebViewControlLongRunningScriptDetectedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNavigationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNavigationCompletedEventArgs[] = L"Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("20409918-4a15-4c46-a55d-f79edb0bde8b")
                IWebViewControlNavigationCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSuccess(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WebErrorStatus(
                        ABI::Windows::Web::WebErrorStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlNavigationCompletedEventArgs = __uuidof(IWebViewControlNavigationCompletedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNavigationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNavigationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNavigationStartingEventArgs[] = L"Windows.Web.UI.IWebViewControlNavigationStartingEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("0c9057c5-0a08-41c7-863b-71e3a9549137")
                IWebViewControlNavigationStartingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Cancel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Cancel(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlNavigationStartingEventArgs = __uuidof(IWebViewControlNavigationStartingEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNewWindowRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("3df44bbb-a124-46d5-a083-d02cacdff5ad")
                IWebViewControlNewWindowRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Referrer(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlNewWindowRequestedEventArgs = __uuidof(IWebViewControlNewWindowRequestedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNewWindowRequestedEventArgs2[] = L"Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("b53c5ca6-2aae-4bfc-92b9-c30e92b48098")
                IWebViewControlNewWindowRequestedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NewWindow(
                        ABI::Windows::Web::UI::IWebViewControl** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NewWindow(
                        ABI::Windows::Web::UI::IWebViewControl* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlNewWindowRequestedEventArgs2 = __uuidof(IWebViewControlNewWindowRequestedEventArgs2);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlPermissionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlPermissionRequest[] = L"Windows.Web.UI.IWebViewControlPermissionRequest";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("e5bc836c-f22f-40e2-95b2-7729f840eb7f")
                IWebViewControlPermissionRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PermissionType(
                        ABI::Windows::Web::UI::WebViewControlPermissionType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Web::UI::WebViewControlPermissionState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Defer(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Allow(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Deny(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlPermissionRequest = __uuidof(IWebViewControlPermissionRequest);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlPermissionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlPermissionRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("27204d51-2488-4cc5-968e-0a771e59c147")
                IWebViewControlPermissionRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PermissionRequest(
                        ABI::Windows::Web::UI::IWebViewControlPermissionRequest** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlPermissionRequestedEventArgs = __uuidof(IWebViewControlPermissionRequestedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlScriptNotifyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlScriptNotifyEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlScriptNotifyEventArgs[] = L"Windows.Web.UI.IWebViewControlScriptNotifyEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("491de57b-6f49-41bb-b591-51b85b817037")
                IWebViewControlScriptNotifyEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlScriptNotifyEventArgs = __uuidof(IWebViewControlScriptNotifyEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlSettings[] = L"Windows.Web.UI.IWebViewControlSettings";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("c9967fbf-5e98-4cfd-8cce-27b0911e3de8")
                IWebViewControlSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_IsJavaScriptEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsJavaScriptEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsIndexedDBEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIndexedDBEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsScriptNotifyAllowed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsScriptNotifyAllowed(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlSettings = __uuidof(IWebViewControlSettings);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs[] = L"Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("e3b81944-e4fc-43dc-94ca-f980f30bc51d")
                IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs = __uuidof(IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlUnviewableContentIdentifiedEventArgs[] = L"Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("4a9680db-88f2-4e20-b693-b4e2df4aa581")
                IWebViewControlUnviewableContentIdentifiedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Referrer(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaType(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlUnviewableContentIdentifiedEventArgs = __uuidof(IWebViewControlUnviewableContentIdentifiedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlWebResourceRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace UI {
                MIDL_INTERFACE("44d6524d-55a4-4d8b-891c-931d8e25d42e")
                IWebViewControlWebResourceRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** deferral
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::Web::Http::IHttpRequestMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Response(
                        ABI::Windows::Web::Http::IHttpResponseMessage* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Response(
                        ABI::Windows::Web::Http::IHttpResponseMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebViewControlWebResourceRequestedEventArgs = __uuidof(IWebViewControlWebResourceRequestedEventArgs);
            } /* UI */
        } /* Web */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlContentLoadingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlContentLoadingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlContentLoadingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlContentLoadingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlContentLoadingEventArgs[] = L"Windows.Web.UI.WebViewControlContentLoadingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs[] = L"Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlDeferredPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlDeferredPermissionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlDeferredPermissionRequest_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlDeferredPermissionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlDeferredPermissionRequest[] = L"Windows.Web.UI.WebViewControlDeferredPermissionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs[] = L"Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs[] = L"Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNavigationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNavigationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNavigationStartingEventArgs[] = L"Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs ** Default Interface **
 *    Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlPermissionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequest_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlPermissionRequest[] = L"Windows.Web.UI.WebViewControlPermissionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlPermissionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlScriptNotifyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlScriptNotifyEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlScriptNotifyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlScriptNotifyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlScriptNotifyEventArgs[] = L"Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlSettings_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlSettings[] = L"Windows.Web.UI.WebViewControlSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs[] = L"Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs[] = L"Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl __x_ABI_CWindows_CWeb_CUI_CIWebViewControl;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2 __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2 __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs;

#endif // ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        __x_ABI_CWindows_CApplicationModel_CDataTransfer_CIDataPackage** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CDataTransfer__CDataPackage_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

typedef struct __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl;

interface __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

typedef struct __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        __FIIterator_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl;

interface __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest;

typedef struct __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        UINT32 index,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl;

interface __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* sender,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CIUriToStreamResolver __x_ABI_CWindows_CWeb_CIUriToStreamResolver;

#endif // ____x_ABI_CWindows_CWeb_CIUriToStreamResolver_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CWeb_CWebErrorStatus __x_ABI_CWindows_CWeb_CWebErrorStatus;

typedef enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionState __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionState;

typedef enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionType __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionType;

/*
 *
 * Struct Windows.Web.UI.WebViewControlPermissionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionState
{
    WebViewControlPermissionState_Unknown = 0,
    WebViewControlPermissionState_Defer = 1,
    WebViewControlPermissionState_Allow = 2,
    WebViewControlPermissionState_Deny = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Web.UI.WebViewControlPermissionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionType
{
    WebViewControlPermissionType_Geolocation = 0,
    WebViewControlPermissionType_UnlimitedIndexedDBQuota = 1,
    WebViewControlPermissionType_Media = 2,
    WebViewControlPermissionType_PointerLock = 3,
    WebViewControlPermissionType_WebNotifications = 4,
    WebViewControlPermissionType_Screen = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    WebViewControlPermissionType_ImmersiveView = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControl[] = L"Windows.Web.UI.IWebViewControl";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Source)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* source);
    HRESULT (STDMETHODCALLTYPE* get_DocumentTitle)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CanGoBack)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanGoForward)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultBackgroundColor)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultBackgroundColor)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ContainsFullScreenElement)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Settings)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings** value);
    HRESULT (STDMETHODCALLTYPE* get_DeferredPermissionRequests)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FIVectorView_1_Windows__CWeb__CUI__CWebViewControlDeferredPermissionRequest** value);
    HRESULT (STDMETHODCALLTYPE* GoForward)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    HRESULT (STDMETHODCALLTYPE* GoBack)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    HRESULT (STDMETHODCALLTYPE* Refresh)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This);
    HRESULT (STDMETHODCALLTYPE* Navigate)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* source);
    HRESULT (STDMETHODCALLTYPE* NavigateToString)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        HSTRING text);
    HRESULT (STDMETHODCALLTYPE* NavigateToLocalStreamUri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* source,
        __x_ABI_CWindows_CWeb_CIUriToStreamResolver* streamResolver);
    HRESULT (STDMETHODCALLTYPE* NavigateWithHttpRequestMessage)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage* requestMessage);
    HRESULT (STDMETHODCALLTYPE* InvokeScriptAsync)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        HSTRING scriptName,
        __FIIterable_1_HSTRING* arguments,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* CapturePreviewToStreamAsync)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* CaptureSelectedContentToDataPackageAsync)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CDataTransfer__CDataPackage** operation);
    HRESULT (STDMETHODCALLTYPE* BuildLocalStreamUri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        HSTRING contentIdentifier,
        HSTRING relativePath,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* GetDeferredPermissionRequestById)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        UINT32 id,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest** result);
    HRESULT (STDMETHODCALLTYPE* add_NavigationStarting)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationStarting)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ContentLoading)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ContentLoading)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DOMContentLoaded)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DOMContentLoaded)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NavigationCompleted)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NavigationCompleted)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FrameNavigationStarting)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationStartingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameNavigationStarting)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FrameContentLoading)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlContentLoadingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameContentLoading)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FrameDOMContentLoaded)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlDOMContentLoadedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameDOMContentLoaded)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_FrameNavigationCompleted)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNavigationCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameNavigationCompleted)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ScriptNotify)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlScriptNotifyEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ScriptNotify)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LongRunningScriptDetected)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlLongRunningScriptDetectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LongRunningScriptDetected)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UnsafeContentWarningDisplaying)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnsafeContentWarningDisplaying)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UnviewableContentIdentified)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnviewableContentIdentifiedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnviewableContentIdentified)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PermissionRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlPermissionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PermissionRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UnsupportedUriSchemeIdentified)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnsupportedUriSchemeIdentified)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_NewWindowRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlNewWindowRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NewWindowRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ContainsFullScreenElementChanged)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ContainsFullScreenElementChanged)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_WebResourceRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        __FITypedEventHandler_2_Windows__CWeb__CUI__CIWebViewControl_Windows__CWeb__CUI__CWebViewControlWebResourceRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_WebResourceRequested)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_put_Source(This, source) \
    ((This)->lpVtbl->put_Source(This, source))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_DocumentTitle(This, value) \
    ((This)->lpVtbl->get_DocumentTitle(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_CanGoBack(This, value) \
    ((This)->lpVtbl->get_CanGoBack(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_CanGoForward(This, value) \
    ((This)->lpVtbl->get_CanGoForward(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_put_DefaultBackgroundColor(This, value) \
    ((This)->lpVtbl->put_DefaultBackgroundColor(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_DefaultBackgroundColor(This, value) \
    ((This)->lpVtbl->get_DefaultBackgroundColor(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_ContainsFullScreenElement(This, value) \
    ((This)->lpVtbl->get_ContainsFullScreenElement(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_Settings(This, value) \
    ((This)->lpVtbl->get_Settings(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_get_DeferredPermissionRequests(This, value) \
    ((This)->lpVtbl->get_DeferredPermissionRequests(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GoForward(This) \
    ((This)->lpVtbl->GoForward(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GoBack(This) \
    ((This)->lpVtbl->GoBack(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_Refresh(This) \
    ((This)->lpVtbl->Refresh(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_Navigate(This, source) \
    ((This)->lpVtbl->Navigate(This, source))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_NavigateToString(This, text) \
    ((This)->lpVtbl->NavigateToString(This, text))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_NavigateToLocalStreamUri(This, source, streamResolver) \
    ((This)->lpVtbl->NavigateToLocalStreamUri(This, source, streamResolver))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_NavigateWithHttpRequestMessage(This, requestMessage) \
    ((This)->lpVtbl->NavigateWithHttpRequestMessage(This, requestMessage))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_InvokeScriptAsync(This, scriptName, arguments, operation) \
    ((This)->lpVtbl->InvokeScriptAsync(This, scriptName, arguments, operation))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_CapturePreviewToStreamAsync(This, stream, operation) \
    ((This)->lpVtbl->CapturePreviewToStreamAsync(This, stream, operation))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_CaptureSelectedContentToDataPackageAsync(This, operation) \
    ((This)->lpVtbl->CaptureSelectedContentToDataPackageAsync(This, operation))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_BuildLocalStreamUri(This, contentIdentifier, relativePath, result) \
    ((This)->lpVtbl->BuildLocalStreamUri(This, contentIdentifier, relativePath, result))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_GetDeferredPermissionRequestById(This, id, result) \
    ((This)->lpVtbl->GetDeferredPermissionRequestById(This, id, result))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_NavigationStarting(This, handler, token) \
    ((This)->lpVtbl->add_NavigationStarting(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_NavigationStarting(This, token) \
    ((This)->lpVtbl->remove_NavigationStarting(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_ContentLoading(This, handler, token) \
    ((This)->lpVtbl->add_ContentLoading(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_ContentLoading(This, token) \
    ((This)->lpVtbl->remove_ContentLoading(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_DOMContentLoaded(This, handler, token) \
    ((This)->lpVtbl->add_DOMContentLoaded(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_DOMContentLoaded(This, token) \
    ((This)->lpVtbl->remove_DOMContentLoaded(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_NavigationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_NavigationCompleted(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_NavigationCompleted(This, token) \
    ((This)->lpVtbl->remove_NavigationCompleted(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_FrameNavigationStarting(This, handler, token) \
    ((This)->lpVtbl->add_FrameNavigationStarting(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_FrameNavigationStarting(This, token) \
    ((This)->lpVtbl->remove_FrameNavigationStarting(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_FrameContentLoading(This, handler, token) \
    ((This)->lpVtbl->add_FrameContentLoading(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_FrameContentLoading(This, token) \
    ((This)->lpVtbl->remove_FrameContentLoading(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_FrameDOMContentLoaded(This, handler, token) \
    ((This)->lpVtbl->add_FrameDOMContentLoaded(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_FrameDOMContentLoaded(This, token) \
    ((This)->lpVtbl->remove_FrameDOMContentLoaded(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_FrameNavigationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_FrameNavigationCompleted(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_FrameNavigationCompleted(This, token) \
    ((This)->lpVtbl->remove_FrameNavigationCompleted(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_ScriptNotify(This, handler, token) \
    ((This)->lpVtbl->add_ScriptNotify(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_ScriptNotify(This, token) \
    ((This)->lpVtbl->remove_ScriptNotify(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_LongRunningScriptDetected(This, handler, token) \
    ((This)->lpVtbl->add_LongRunningScriptDetected(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_LongRunningScriptDetected(This, token) \
    ((This)->lpVtbl->remove_LongRunningScriptDetected(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_UnsafeContentWarningDisplaying(This, handler, token) \
    ((This)->lpVtbl->add_UnsafeContentWarningDisplaying(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_UnsafeContentWarningDisplaying(This, token) \
    ((This)->lpVtbl->remove_UnsafeContentWarningDisplaying(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_UnviewableContentIdentified(This, handler, token) \
    ((This)->lpVtbl->add_UnviewableContentIdentified(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_UnviewableContentIdentified(This, token) \
    ((This)->lpVtbl->remove_UnviewableContentIdentified(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_PermissionRequested(This, handler, token) \
    ((This)->lpVtbl->add_PermissionRequested(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_PermissionRequested(This, token) \
    ((This)->lpVtbl->remove_PermissionRequested(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_UnsupportedUriSchemeIdentified(This, handler, token) \
    ((This)->lpVtbl->add_UnsupportedUriSchemeIdentified(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_UnsupportedUriSchemeIdentified(This, token) \
    ((This)->lpVtbl->remove_UnsupportedUriSchemeIdentified(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_NewWindowRequested(This, handler, token) \
    ((This)->lpVtbl->add_NewWindowRequested(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_NewWindowRequested(This, token) \
    ((This)->lpVtbl->remove_NewWindowRequested(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_ContainsFullScreenElementChanged(This, handler, token) \
    ((This)->lpVtbl->add_ContainsFullScreenElementChanged(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_ContainsFullScreenElementChanged(This, token) \
    ((This)->lpVtbl->remove_ContainsFullScreenElementChanged(This, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_add_WebResourceRequested(This, handler, token) \
    ((This)->lpVtbl->add_WebResourceRequested(This, handler, token))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl_remove_WebResourceRequested(This, token) \
    ((This)->lpVtbl->remove_WebResourceRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControl;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControl2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControl2[] = L"Windows.Web.UI.IWebViewControl2";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddInitializeScript)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControl2* This,
        HSTRING script);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2Vtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_AddInitializeScript(This, script) \
    ((This)->lpVtbl->AddInitializeScript(This, script))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControl2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControl2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlContentLoadingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlContentLoadingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlContentLoadingEventArgs[] = L"Windows.Web.UI.IWebViewControlContentLoadingEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlContentLoadingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlDOMContentLoadedEventArgs[] = L"Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDOMContentLoadedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlDeferredPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlDeferredPermissionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlDeferredPermissionRequest[] = L"Windows.Web.UI.IWebViewControlDeferredPermissionRequest";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_PermissionType)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This,
        enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionType* value);
    HRESULT (STDMETHODCALLTYPE* Allow)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* Deny)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest* This);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequestVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_get_PermissionType(This, value) \
    ((This)->lpVtbl->get_PermissionType(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_Allow(This) \
    ((This)->lpVtbl->Allow(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_Deny(This) \
    ((This)->lpVtbl->Deny(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlDeferredPermissionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlLongRunningScriptDetectedEventArgs[] = L"Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExecutionTime)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_StopPageScriptExecution)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_StopPageScriptExecution)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_get_ExecutionTime(This, value) \
    ((This)->lpVtbl->get_ExecutionTime(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_get_StopPageScriptExecution(This, value) \
    ((This)->lpVtbl->get_StopPageScriptExecution(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_put_StopPageScriptExecution(This, value) \
    ((This)->lpVtbl->put_StopPageScriptExecution(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlLongRunningScriptDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNavigationCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNavigationCompletedEventArgs[] = L"Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_IsSuccess)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WebErrorStatus)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_get_IsSuccess(This, value) \
    ((This)->lpVtbl->get_IsSuccess(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_get_WebErrorStatus(This, value) \
    ((This)->lpVtbl->get_WebErrorStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNavigationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNavigationStartingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNavigationStartingEventArgs[] = L"Windows.Web.UI.IWebViewControlNavigationStartingEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Cancel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Cancel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_get_Cancel(This, value) \
    ((This)->lpVtbl->get_Cancel(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_put_Cancel(This, value) \
    ((This)->lpVtbl->put_Cancel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNavigationStartingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNewWindowRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Referrer)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_get_Referrer(This, value) \
    ((This)->lpVtbl->get_Referrer(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlNewWindowRequestedEventArgs2[] = L"Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NewWindow)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl** value);
    HRESULT (STDMETHODCALLTYPE* put_NewWindow)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControl* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2Vtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_get_NewWindow(This, value) \
    ((This)->lpVtbl->get_NewWindow(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_put_NewWindow(This, value) \
    ((This)->lpVtbl->put_NewWindow(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlNewWindowRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlPermissionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlPermissionRequest[] = L"Windows.Web.UI.IWebViewControlPermissionRequest";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_PermissionType)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionType* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This,
        enum __x_ABI_CWindows_CWeb_CUI_CWebViewControlPermissionState* value);
    HRESULT (STDMETHODCALLTYPE* Defer)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* Allow)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This);
    HRESULT (STDMETHODCALLTYPE* Deny)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest* This);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_get_PermissionType(This, value) \
    ((This)->lpVtbl->get_PermissionType(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_Defer(This) \
    ((This)->lpVtbl->Defer(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_Allow(This) \
    ((This)->lpVtbl->Allow(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_Deny(This) \
    ((This)->lpVtbl->Deny(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlPermissionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlPermissionRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PermissionRequest)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_get_PermissionRequest(This, value) \
    ((This)->lpVtbl->get_PermissionRequest(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlPermissionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlScriptNotifyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlScriptNotifyEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlScriptNotifyEventArgs[] = L"Windows.Web.UI.IWebViewControlScriptNotifyEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlScriptNotifyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlSettings[] = L"Windows.Web.UI.IWebViewControlSettings";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_IsJavaScriptEnabled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsJavaScriptEnabled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsIndexedDBEnabled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsIndexedDBEnabled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsScriptNotifyAllowed)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsScriptNotifyAllowed)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettingsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_put_IsJavaScriptEnabled(This, value) \
    ((This)->lpVtbl->put_IsJavaScriptEnabled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_get_IsJavaScriptEnabled(This, value) \
    ((This)->lpVtbl->get_IsJavaScriptEnabled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_put_IsIndexedDBEnabled(This, value) \
    ((This)->lpVtbl->put_IsIndexedDBEnabled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_get_IsIndexedDBEnabled(This, value) \
    ((This)->lpVtbl->get_IsIndexedDBEnabled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_put_IsScriptNotifyAllowed(This, value) \
    ((This)->lpVtbl->put_IsScriptNotifyAllowed(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_get_IsScriptNotifyAllowed(This, value) \
    ((This)->lpVtbl->get_IsScriptNotifyAllowed(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs[] = L"Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlUnviewableContentIdentifiedEventArgs[] = L"Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Referrer)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_MediaType)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_get_Referrer(This, value) \
    ((This)->lpVtbl->get_Referrer(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_get_MediaType(This, value) \
    ((This)->lpVtbl->get_MediaType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlUnviewableContentIdentifiedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Web_UI_IWebViewControlWebResourceRequestedEventArgs[] = L"Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs";
typedef struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** deferral);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpRequestMessage** value);
    HRESULT (STDMETHODCALLTYPE* put_Response)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage* value);
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_put_Response(This, value) \
    ((This)->lpVtbl->put_Response(This, value))

#define __x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_get_Response(This, value) \
    ((This)->lpVtbl->get_Response(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CWeb_CUI_CIWebViewControlWebResourceRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlContentLoadingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlContentLoadingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlContentLoadingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlContentLoadingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlContentLoadingEventArgs[] = L"Windows.Web.UI.WebViewControlContentLoadingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlDOMContentLoadedEventArgs[] = L"Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlDeferredPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlDeferredPermissionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlDeferredPermissionRequest_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlDeferredPermissionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlDeferredPermissionRequest[] = L"Windows.Web.UI.WebViewControlDeferredPermissionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlLongRunningScriptDetectedEventArgs[] = L"Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNavigationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNavigationCompletedEventArgs[] = L"Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNavigationStartingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNavigationStartingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationStartingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNavigationStartingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNavigationStartingEventArgs[] = L"Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs ** Default Interface **
 *    Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlNewWindowRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlPermissionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlPermissionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequest_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlPermissionRequest[] = L"Windows.Web.UI.WebViewControlPermissionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlPermissionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlPermissionRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlScriptNotifyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlScriptNotifyEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlScriptNotifyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlScriptNotifyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlScriptNotifyEventArgs[] = L"Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlSettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlSettings_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlSettings[] = L"Windows.Web.UI.WebViewControlSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlUnsupportedUriSchemeIdentifiedEventArgs[] = L"Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlUnviewableContentIdentifiedEventArgs[] = L"Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Web_UI_WebViewControlWebResourceRequestedEventArgs[] = L"Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eweb2Eui_p_h__

#endif // __windows2Eweb2Eui_h__
