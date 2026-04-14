
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
#ifndef __windows2Eui2Ewindowmanagement_h__
#define __windows2Eui2Ewindowmanagement_h__
#ifndef __windows2Eui2Ewindowmanagement_p_h__
#define __windows2Eui2Ewindowmanagement_p_h__


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
#include "Windows.UI.h"
#include "Windows.UI.Composition.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow ABI::Windows::UI::WindowManagement::IAppWindow

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowChangedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs ABI::Windows::UI::WindowManagement::IAppWindowChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowCloseRequestedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs ABI::Windows::UI::WindowManagement::IAppWindowCloseRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowClosedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs ABI::Windows::UI::WindowManagement::IAppWindowClosedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowFrame;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame ABI::Windows::UI::WindowManagement::IAppWindowFrame

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowFrameStyle;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle ABI::Windows::UI::WindowManagement::IAppWindowFrameStyle

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowPlacement;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement ABI::Windows::UI::WindowManagement::IAppWindowPlacement

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowPresentationConfiguration;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration ABI::Windows::UI::WindowManagement::IAppWindowPresentationConfiguration

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowPresentationConfigurationFactory;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory ABI::Windows::UI::WindowManagement::IAppWindowPresentationConfigurationFactory

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowPresenter;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter ABI::Windows::UI::WindowManagement::IAppWindowPresenter

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowStatics;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics ABI::Windows::UI::WindowManagement::IAppWindowStatics

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowTitleBar;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar ABI::Windows::UI::WindowManagement::IAppWindowTitleBar

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowTitleBarOcclusion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion ABI::Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IAppWindowTitleBarVisibility;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility ABI::Windows::UI::WindowManagement::IAppWindowTitleBarVisibility

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface ICompactOverlayPresentationConfiguration;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration ABI::Windows::UI::WindowManagement::ICompactOverlayPresentationConfiguration

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IDefaultPresentationConfiguration;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration ABI::Windows::UI::WindowManagement::IDefaultPresentationConfiguration

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IDisplayRegion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion ABI::Windows::UI::WindowManagement::IDisplayRegion

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IFullScreenPresentationConfiguration;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration ABI::Windows::UI::WindowManagement::IFullScreenPresentationConfiguration

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowServicesStatics;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics ABI::Windows::UI::WindowManagement::IWindowServicesStatics

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowingEnvironment;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment ABI::Windows::UI::WindowManagement::IWindowingEnvironment

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowingEnvironmentAddedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs ABI::Windows::UI::WindowManagement::IWindowingEnvironmentAddedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowingEnvironmentChangedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs ABI::Windows::UI::WindowManagement::IWindowingEnvironmentChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowingEnvironmentRemovedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs ABI::Windows::UI::WindowManagement::IWindowingEnvironmentRemovedEventArgs

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IWindowingEnvironmentStatics;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics ABI::Windows::UI::WindowManagement::IWindowingEnvironmentStatics

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__

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
        namespace UI {
            namespace WindowManagement {
                class AppWindow;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4167727a-5df0-5ed3-b624-167c81beff6b"))
IAsyncOperation<ABI::Windows::UI::WindowManagement::AppWindow*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::IAppWindow*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.WindowManagement.AppWindow>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::WindowManagement::AppWindow*> __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_t;
#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3bbea3e6-34b8-5ef0-a093-9cc71fa40b6e"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::WindowManagement::AppWindow*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::IAppWindow*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.WindowManagement.AppWindow>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::WindowManagement::AppWindow*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface IVisualElement;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CIVisualElement ABI::Windows::UI::Composition::IVisualElement

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CComposition__CIVisualElement_USE
#define DEF___FIIterator_1_Windows__CUI__CComposition__CIVisualElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3142fdb3-4110-5819-b966-9c2a172e209f"))
IIterator<ABI::Windows::UI::Composition::IVisualElement*> : IIterator_impl<ABI::Windows::UI::Composition::IVisualElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Composition.IVisualElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Composition::IVisualElement*> __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_t;
#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CComposition__CIVisualElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CComposition__CIVisualElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CComposition__CIVisualElement_USE
#define DEF___FIIterable_1_Windows__CUI__CComposition__CIVisualElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a0394077-1a66-589c-997d-2acba9051f77"))
IIterable<ABI::Windows::UI::Composition::IVisualElement*> : IIterable_impl<ABI::Windows::UI::Composition::IVisualElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Composition.IVisualElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Composition::IVisualElement*> __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_t;
#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CComposition__CIVisualElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CComposition__CIVisualElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIIterator_1_Windows__CUI__CWindowId_USE
#define DEF___FIIterator_1_Windows__CUI__CWindowId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ba0a30a1-c082-5671-ac07-7aaa4f269670"))
IIterator<struct ABI::Windows::UI::WindowId> : IIterator_impl<struct ABI::Windows::UI::WindowId>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.WindowId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::UI::WindowId> __FIIterator_1_Windows__CUI__CWindowId_t;
#define __FIIterator_1_Windows__CUI__CWindowId ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CWindowId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CWindowId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIIterable_1_Windows__CUI__CWindowId_USE
#define DEF___FIIterable_1_Windows__CUI__CWindowId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("745698bf-22ad-5c0d-b0e0-07d35a1c9719"))
IIterable<struct ABI::Windows::UI::WindowId> : IIterable_impl<struct ABI::Windows::UI::WindowId>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.WindowId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::UI::WindowId> __FIIterable_1_Windows__CUI__CWindowId_t;
#define __FIIterable_1_Windows__CUI__CWindowId ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CWindowId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CWindowId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowTitleBarOcclusion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#define DEF___FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("13e952db-f553-5c43-8fd2-0e1b9df3ee3f"))
IIterator<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*, ABI::Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.WindowManagement.AppWindowTitleBarOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t;
#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#define DEF___FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3503c7c3-d036-5152-92dd-de9732205420"))
IIterable<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*, ABI::Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.WindowManagement.AppWindowTitleBarOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t;
#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class DisplayRegion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#define DEF___FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("110d4d7e-2f29-51b8-9691-8b206ad1d73b"))
IIterator<ABI::Windows::UI::WindowManagement::DisplayRegion*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::DisplayRegion*, ABI::Windows::UI::WindowManagement::IDisplayRegion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.WindowManagement.DisplayRegion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::WindowManagement::DisplayRegion*> __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_t;
#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#define DEF___FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("026730ab-250d-503c-a876-43bbb754ad44"))
IIterable<ABI::Windows::UI::WindowManagement::DisplayRegion*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::DisplayRegion*, ABI::Windows::UI::WindowManagement::IDisplayRegion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.WindowManagement.DisplayRegion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::WindowManagement::DisplayRegion*> __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_t;
#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class WindowingEnvironment;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#define DEF___FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a35c192a-9459-5bcc-9db3-23243716197f"))
IIterator<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::IWindowingEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.WindowManagement.WindowingEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t;
#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#define DEF___FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("626a6481-0174-5a56-84c9-e8d4c48cfb1b"))
IIterable<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::IWindowingEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.WindowManagement.WindowingEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t;
#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_USE
#define DEF___FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("48e605a6-1fa6-5b4b-a802-17f54c4bcccc"))
IVectorView<ABI::Windows::UI::Composition::IVisualElement*> : IVectorView_impl<ABI::Windows::UI::Composition::IVisualElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Composition.IVisualElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Composition::IVisualElement*> __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_t;
#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#ifndef DEF___FIVectorView_1_Windows__CUI__CWindowId_USE
#define DEF___FIVectorView_1_Windows__CUI__CWindowId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f49e7371-b31a-5620-a42e-7e969003f0ff"))
IVectorView<struct ABI::Windows::UI::WindowId> : IVectorView_impl<struct ABI::Windows::UI::WindowId>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.WindowId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::UI::WindowId> __FIVectorView_1_Windows__CUI__CWindowId_t;
#define __FIVectorView_1_Windows__CUI__CWindowId ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CWindowId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CWindowId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#define DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("586d5577-7d02-5c77-bb43-39522f80972e"))
IVectorView<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*, ABI::Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.WindowManagement.AppWindowTitleBarOcclusion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::WindowManagement::AppWindowTitleBarOcclusion*> __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t;
#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#define DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9460debb-a0d6-5ab8-84f5-9087f2a6bb67"))
IVectorView<ABI::Windows::UI::WindowManagement::DisplayRegion*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::DisplayRegion*, ABI::Windows::UI::WindowManagement::IDisplayRegion*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.WindowManagement.DisplayRegion>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::WindowManagement::DisplayRegion*> __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_t;
#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#define DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c833c1c0-79c6-522e-b71b-d845e0f6668e"))
IVectorView<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::IWindowingEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.WindowManagement.WindowingEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::WindowManagement::WindowingEnvironment*> __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t;
#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVector_1_Windows__CUI__CComposition__CIVisualElement_USE
#define DEF___FIVector_1_Windows__CUI__CComposition__CIVisualElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf59c70b-6d34-55fb-9313-9781433e778a"))
IVector<ABI::Windows::UI::Composition::IVisualElement*> : IVector_impl<ABI::Windows::UI::Composition::IVisualElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Composition.IVisualElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Composition::IVisualElement*> __FIVector_1_Windows__CUI__CComposition__CIVisualElement_t;
#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CComposition__CIVisualElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CComposition__CIVisualElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CUI__CColor_USE
#define DEF___FIReference_1_Windows__CUI__CColor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab8e5d11-b0c1-5a21-95ae-f16bf3a37624"))
IReference<struct ABI::Windows::UI::Color> : IReference_impl<struct ABI::Windows::UI::Color>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.UI.Color>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::UI::Color> __FIReference_1_Windows__CUI__CColor_t;
#define __FIReference_1_Windows__CUI__CColor ABI::Windows::Foundation::__FIReference_1_Windows__CUI__CColor_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CUI__CColor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowChangedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c6a30887-6f60-597f-b918-757cab5f3a76"))
ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::IAppWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowChangedEventArgs*, ABI::Windows::UI::WindowManagement::IAppWindowChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WindowManagement.AppWindow, Windows.UI.WindowManagement.AppWindowChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowCloseRequestedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c0359718-173d-5f19-b76a-91929b2b0451"))
ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowCloseRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::IAppWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowCloseRequestedEventArgs*, ABI::Windows::UI::WindowManagement::IAppWindowCloseRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WindowManagement.AppWindow, Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowCloseRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowClosedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7f62c430-89b3-5873-93ca-bb4d71a390e4"))
ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowClosedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::IAppWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::AppWindowClosedEventArgs*, ABI::Windows::UI::WindowManagement::IAppWindowClosedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WindowManagement.AppWindow, Windows.UI.WindowManagement.AppWindowClosedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WindowManagement::AppWindow*, ABI::Windows::UI::WindowManagement::AppWindowClosedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c40d18b-7438-5eb4-9359-7897fce7e3fc"))
ITypedEventHandler<ABI::Windows::UI::WindowManagement::DisplayRegion*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::DisplayRegion*, ABI::Windows::UI::WindowManagement::IDisplayRegion*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WindowManagement.DisplayRegion, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WindowManagement::DisplayRegion*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class WindowingEnvironmentChangedEventArgs;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3af2bd46-4225-5a93-85ed-73a01e91d0b5"))
ITypedEventHandler<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::WindowingEnvironmentChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::IWindowingEnvironment*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::WindowManagement::WindowingEnvironmentChangedEventArgs*, ABI::Windows::UI::WindowManagement::IWindowingEnvironmentChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.WindowManagement.WindowingEnvironment, Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::WindowManagement::WindowingEnvironment*, ABI::Windows::UI::WindowManagement::WindowingEnvironmentChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace UI {
            class UIContentRoot;
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContentRoot;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContentRoot ABI::Windows::UI::IUIContentRoot

#endif // ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            class UIContext;
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContext;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContext ABI::Windows::UI::IUIContext

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                typedef enum AppWindowClosedReason : int AppWindowClosedReason;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                typedef enum AppWindowFrameStyle : int AppWindowFrameStyle;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                typedef enum AppWindowPresentationKind : int AppWindowPresentationKind;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                typedef enum AppWindowTitleBarVisibility : int AppWindowTitleBarVisibility;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                typedef enum WindowingEnvironmentKind : int WindowingEnvironmentKind;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowFrame;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowPlacement;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowPresentationConfiguration;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowPresenter;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                class AppWindowTitleBar;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                enum AppWindowClosedReason : int
                {
                    AppWindowClosedReason_Other = 0,
                    AppWindowClosedReason_AppInitiated = 1,
                    AppWindowClosedReason_UserInitiated = 2,
                };
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowFrameStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                enum AppWindowFrameStyle : int
                {
                    AppWindowFrameStyle_Default = 0,
                    AppWindowFrameStyle_NoFrame = 1,
                };
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowPresentationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                enum AppWindowPresentationKind : int
                {
                    AppWindowPresentationKind_Default = 0,
                    AppWindowPresentationKind_CompactOverlay = 1,
                    AppWindowPresentationKind_FullScreen = 2,
                };
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowTitleBarVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                enum AppWindowTitleBarVisibility : int
                {
                    AppWindowTitleBarVisibility_Default = 0,
                    AppWindowTitleBarVisibility_AlwaysHidden = 1,
                };
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.WindowingEnvironmentKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                enum WindowingEnvironmentKind : int
                {
                    WindowingEnvironmentKind_Unknown = 0,
                    WindowingEnvironmentKind_Overlapped = 1,
                    WindowingEnvironmentKind_Tiled = 2,
                };
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindow[] = L"Windows.UI.WindowManagement.IAppWindow";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("663014a6-b75e-5dbd-995c-f0117fa3fb61")
                IAppWindow : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::UI::IUIContentRoot** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Frame(
                        ABI::Windows::UI::WindowManagement::IAppWindowFrame** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PersistedStateId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PersistedStateId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Presenter(
                        ABI::Windows::UI::WindowManagement::IAppWindowPresenter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TitleBar(
                        ABI::Windows::UI::WindowManagement::IAppWindowTitleBar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UIContext(
                        ABI::Windows::UI::IUIContext** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WindowingEnvironment(
                        ABI::Windows::UI::WindowManagement::IWindowingEnvironment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CloseAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPlacement(
                        ABI::Windows::UI::WindowManagement::IAppWindowPlacement** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDisplayRegions(
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveToDisplayRegion(
                        ABI::Windows::UI::WindowManagement::IDisplayRegion* displayRegion
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveAdjacentToCurrentView(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveAdjacentToWindow(
                        ABI::Windows::UI::WindowManagement::IAppWindow* anchorWindow
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveRelativeToWindowContent(
                        ABI::Windows::UI::WindowManagement::IAppWindow* anchorWindow,
                        ABI::Windows::Foundation::Point contentOffset
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveRelativeToCurrentViewContent(
                        ABI::Windows::Foundation::Point contentOffset
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestMoveRelativeToDisplayRegion(
                        ABI::Windows::UI::WindowManagement::IDisplayRegion* displayRegion,
                        ABI::Windows::Foundation::Point displayRegionOffset
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestSize(
                        ABI::Windows::Foundation::Size frameSize
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CloseRequested(
                        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CloseRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindow = __uuidof(IAppWindow);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowChangedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("1de1f3be-a655-55ad-b2b6-eb240f880356")
                IAppWindowChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DidAvailableWindowPresentationsChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidDisplayRegionsChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidFrameChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidSizeChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidTitleBarChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidVisibilityChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidWindowingEnvironmentChange(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DidWindowPresentationChange(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowChangedEventArgs = __uuidof(IAppWindowChangedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowCloseRequestedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("e9ff01da-e7a2-57a8-8b5e-39c4003afdbb")
                IAppWindowCloseRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Cancel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Cancel(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowCloseRequestedEventArgs = __uuidof(IAppWindowCloseRequestedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowClosedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowClosedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("cc7df816-9520-5a06-821e-456ad8b358aa")
                IAppWindowClosedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::UI::WindowManagement::AppWindowClosedReason* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowClosedEventArgs = __uuidof(IAppWindowClosedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowFrame[] = L"Windows.UI.WindowManagement.IAppWindowFrame";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("9ee22601-7e5d-52af-846b-01dc6c296567")
                IAppWindowFrame : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DragRegionVisuals(
                        __FIVector_1_Windows__CUI__CComposition__CIVisualElement** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowFrame = __uuidof(IAppWindowFrame);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowFrameStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowFrameStyle[] = L"Windows.UI.WindowManagement.IAppWindowFrameStyle";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("ac412946-e1ac-5230-944a-c60873dcf4a9")
                IAppWindowFrameStyle : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetFrameStyle(
                        ABI::Windows::UI::WindowManagement::AppWindowFrameStyle* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFrameStyle(
                        ABI::Windows::UI::WindowManagement::AppWindowFrameStyle frameStyle
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowFrameStyle = __uuidof(IAppWindowFrameStyle);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPlacement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPlacement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPlacement[] = L"Windows.UI.WindowManagement.IAppWindowPlacement";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("03dc815e-e7a9-5857-9c03-7d670594410e")
                IAppWindowPlacement : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayRegion(
                        ABI::Windows::UI::WindowManagement::IDisplayRegion** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Offset(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowPlacement = __uuidof(IAppWindowPlacement);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresentationConfiguration[] = L"Windows.UI.WindowManagement.IAppWindowPresentationConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("b5a43ee3-df33-5e67-bd31-1072457300df")
                IAppWindowPresentationConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::UI::WindowManagement::AppWindowPresentationKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowPresentationConfiguration = __uuidof(IAppWindowPresentationConfiguration);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresentationConfigurationFactory[] = L"Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("fd3606a6-7875-5de8-84ff-6351ee13dd0d")
                IAppWindowPresentationConfigurationFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IAppWindowPresentationConfigurationFactory = __uuidof(IAppWindowPresentationConfigurationFactory);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresenter[] = L"Windows.UI.WindowManagement.IAppWindowPresenter";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("5ae9ed73-e1fd-5317-ad78-5a3ed271bbde")
                IAppWindowPresenter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetConfiguration(
                        ABI::Windows::UI::WindowManagement::IAppWindowPresentationConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsPresentationSupported(
                        ABI::Windows::UI::WindowManagement::AppWindowPresentationKind presentationKind,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPresentation(
                        ABI::Windows::UI::WindowManagement::IAppWindowPresentationConfiguration* configuration,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestPresentationByKind(
                        ABI::Windows::UI::WindowManagement::AppWindowPresentationKind presentationKind,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowPresenter = __uuidof(IAppWindowPresenter);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowStatics[] = L"Windows.UI.WindowManagement.IAppWindowStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("ff1f3ea3-b769-50ef-9873-108cd0e89746")
                IAppWindowStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryCreateAsync(
                        __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearAllPersistedState(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearPersistedState(
                        HSTRING key
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowStatics = __uuidof(IAppWindowStatics);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBar[] = L"Windows.UI.WindowManagement.IAppWindowTitleBar";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("6e932c84-f644-541d-a2d7-0c262437842d")
                IAppWindowTitleBar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonHoverBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonHoverBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonHoverForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonHoverForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonInactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonInactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonInactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonInactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonPressedBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonPressedBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonPressedForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonPressedForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendsContentIntoTitleBar(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExtendsContentIntoTitleBar(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTitleBarOcclusions(
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowTitleBar = __uuidof(IAppWindowTitleBar);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBarOcclusion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBarOcclusion[] = L"Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("fea3cffd-2ccf-5fc3-aeae-f843876bf37e")
                IAppWindowTitleBarOcclusion : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OccludingRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowTitleBarOcclusion = __uuidof(IAppWindowTitleBarOcclusion);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBarVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBarVisibility[] = L"Windows.UI.WindowManagement.IAppWindowTitleBarVisibility";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("a215a4e3-6e7e-5651-8c3b-624819528154")
                IAppWindowTitleBarVisibility : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPreferredVisibility(
                        ABI::Windows::UI::WindowManagement::AppWindowTitleBarVisibility* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPreferredVisibility(
                        ABI::Windows::UI::WindowManagement::AppWindowTitleBarVisibility visibilityMode
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppWindowTitleBarVisibility = __uuidof(IAppWindowTitleBarVisibility);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_ICompactOverlayPresentationConfiguration[] = L"Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("a7e5750f-5730-56c6-8e1f-d63ff4d7980d")
                ICompactOverlayPresentationConfiguration : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ICompactOverlayPresentationConfiguration = __uuidof(ICompactOverlayPresentationConfiguration);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IDefaultPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.DefaultPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IDefaultPresentationConfiguration[] = L"Windows.UI.WindowManagement.IDefaultPresentationConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("d8c2b53b-2168-5703-a853-d525589fe2b9")
                IDefaultPresentationConfiguration : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IDefaultPresentationConfiguration = __uuidof(IDefaultPresentationConfiguration);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IDisplayRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.DisplayRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IDisplayRegion[] = L"Windows.UI.WindowManagement.IDisplayRegion";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("db50c3a2-4094-5f47-8cb1-ea01ddafaa94")
                IDisplayRegion : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayMonitorDeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WorkAreaOffset(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WorkAreaSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WindowingEnvironment(
                        ABI::Windows::UI::WindowManagement::IWindowingEnvironment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayRegion = __uuidof(IDisplayRegion);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IFullScreenPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.FullScreenPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IFullScreenPresentationConfiguration[] = L"Windows.UI.WindowManagement.IFullScreenPresentationConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("43d3dcd8-d2a8-503d-a626-15533d6d5f62")
                IFullScreenPresentationConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsExclusive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsExclusive(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFullScreenPresentationConfiguration = __uuidof(IFullScreenPresentationConfiguration);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowServicesStatics[] = L"Windows.UI.WindowManagement.IWindowServicesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("cff4d519-50a6-5c64-97f6-c2d96add7f42")
                IWindowServicesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAllTopLevelWindowIds(
                        __FIVectorView_1_Windows__CUI__CWindowId** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowServicesStatics = __uuidof(IWindowServicesStatics);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironment[] = L"Windows.UI.WindowManagement.IWindowingEnvironment";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("264363c0-2a49-5417-b3ae-48a71c63a3bd")
                IWindowingEnvironment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::UI::WindowManagement::WindowingEnvironmentKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDisplayRegions(
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowingEnvironment = __uuidof(IWindowingEnvironment);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentAddedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("ff2a5b7f-f183-5c66-99b2-429082069299")
                IWindowingEnvironmentAddedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowingEnvironment(
                        ABI::Windows::UI::WindowManagement::IWindowingEnvironment** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowingEnvironmentAddedEventArgs = __uuidof(IWindowingEnvironmentAddedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentChangedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("4160cfc6-023d-5e9a-b431-350e67dc978a")
                IWindowingEnvironmentChangedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IWindowingEnvironmentChangedEventArgs = __uuidof(IWindowingEnvironmentChangedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentRemovedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("2e5b5473-beff-5e53-9316-7e775fe568b3")
                IWindowingEnvironmentRemovedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowingEnvironment(
                        ABI::Windows::UI::WindowManagement::IWindowingEnvironment** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowingEnvironmentRemovedEventArgs = __uuidof(IWindowingEnvironmentRemovedEventArgs);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentStatics[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                MIDL_INTERFACE("874e9fb7-c642-55ab-8aa2-162f734a9a72")
                IWindowingEnvironmentStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAll(
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllWithKind(
                        ABI::Windows::UI::WindowManagement::WindowingEnvironmentKind kind,
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowingEnvironmentStatics = __uuidof(IWindowingEnvironmentStatics);
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IAppWindowStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindow ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindow[] = L"Windows.UI.WindowManagement.AppWindow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowChangedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowClosedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowFrameStyle
 *    Windows.UI.WindowManagement.IAppWindowFrame ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowFrame_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowFrame[] = L"Windows.UI.WindowManagement.AppWindowFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPlacement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPlacement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPlacement_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPlacement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPlacement[] = L"Windows.UI.WindowManagement.AppWindowPlacement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPresentationConfiguration[] = L"Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPresenter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPresenter[] = L"Windows.UI.WindowManagement.AppWindowPresenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowTitleBarVisibility
 *    Windows.UI.WindowManagement.IAppWindowTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBar_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowTitleBar[] = L"Windows.UI.WindowManagement.AppWindowTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowTitleBarOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion[] = L"Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration[] = L"Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.DefaultPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IDefaultPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_DefaultPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_DefaultPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_DefaultPresentationConfiguration[] = L"Windows.UI.WindowManagement.DefaultPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.DisplayRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IDisplayRegion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_DisplayRegion_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_DisplayRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_DisplayRegion[] = L"Windows.UI.WindowManagement.DisplayRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.FullScreenPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IFullScreenPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_FullScreenPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_FullScreenPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_FullScreenPresentationConfiguration[] = L"Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IWindowServicesStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowServices_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowServices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowServices[] = L"Windows.UI.WindowManagement.WindowServices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IWindowingEnvironmentStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironment_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironment[] = L"Windows.UI.WindowManagement.WindowingEnvironment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow;

typedef struct __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindowVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow* This,
        __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindowVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CWindowManagement__CAppWindow_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisualElement __x_ABI_CWindows_CUI_CComposition_CIVisualElement;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CComposition__CIVisualElement __FIIterator_1_Windows__CUI__CComposition__CIVisualElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CComposition__CIVisualElement;

typedef struct __FIIterator_1_Windows__CUI__CComposition__CIVisualElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CComposition__CIVisualElementVtbl;

interface __FIIterator_1_Windows__CUI__CComposition__CIVisualElement
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CComposition__CIVisualElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CComposition__CIVisualElement_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CComposition__CIVisualElement __FIIterable_1_Windows__CUI__CComposition__CIVisualElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CComposition__CIVisualElement;

typedef struct __FIIterable_1_Windows__CUI__CComposition__CIVisualElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CComposition__CIVisualElement* This,
        __FIIterator_1_Windows__CUI__CComposition__CIVisualElement** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CComposition__CIVisualElementVtbl;

interface __FIIterable_1_Windows__CUI__CComposition__CIVisualElement
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CComposition__CIVisualElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CComposition__CIVisualElement_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIIterator_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CWindowId __FIIterator_1_Windows__CUI__CWindowId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CWindowId;

typedef struct __FIIterator_1_Windows__CUI__CWindowIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CWindowId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CWindowId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CWindowId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CWindowId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CWindowId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CWindowId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CWindowId* This,
        struct __x_ABI_CWindows_CUI_CWindowId* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CWindowId* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CWindowId* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CWindowId* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CWindowId* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CWindowIdVtbl;

interface __FIIterator_1_Windows__CUI__CWindowId
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CWindowIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CWindowId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CWindowId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CWindowId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CWindowId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CWindowId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CWindowId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CWindowId_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CWindowId_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CWindowId_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CWindowId_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIIterable_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CWindowId __FIIterable_1_Windows__CUI__CWindowId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CWindowId;

typedef struct __FIIterable_1_Windows__CUI__CWindowIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CWindowId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CWindowId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CWindowId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CWindowId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CWindowId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CWindowId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CWindowId* This,
        __FIIterator_1_Windows__CUI__CWindowId** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CWindowIdVtbl;

interface __FIIterable_1_Windows__CUI__CWindowId
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CWindowIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CWindowId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CWindowId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CWindowId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CWindowId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CWindowId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CWindowId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CWindowId_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

typedef struct __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl;

interface __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

typedef struct __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        __FIIterator_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl;

interface __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion;

typedef struct __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl;

interface __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion;

typedef struct __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        __FIIterator_1_Windows__CUI__CWindowManagement__CDisplayRegion** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl;

interface __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

typedef struct __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl;

interface __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

typedef struct __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        __FIIterator_1_Windows__CUI__CWindowManagement__CWindowingEnvironment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl;

interface __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CComposition__CIVisualElement;

typedef struct __FIVectorView_1_Windows__CUI__CComposition__CIVisualElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CComposition__CIVisualElementVtbl;

interface __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CComposition__CIVisualElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____FIVectorView_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CWindowId __FIVectorView_1_Windows__CUI__CWindowId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CWindowId;

typedef struct __FIVectorView_1_Windows__CUI__CWindowIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CWindowId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CWindowId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        UINT32 index,
        struct __x_ABI_CWindows_CUI_CWindowId* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        struct __x_ABI_CWindows_CUI_CWindowId value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CWindowId* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CWindowId* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CWindowIdVtbl;

interface __FIVectorView_1_Windows__CUI__CWindowId
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CWindowIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CWindowId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CWindowId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CWindowId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CWindowId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CWindowId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CWindowId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CWindowId_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowId_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CWindowId_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowId_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CWindowId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion;

typedef struct __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl;

interface __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion;

typedef struct __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl;

interface __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment;

typedef struct __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl;

interface __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVector_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CComposition__CIVisualElement __FIVector_1_Windows__CUI__CComposition__CIVisualElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CComposition__CIVisualElement;

typedef struct __FIVector_1_Windows__CUI__CComposition__CIVisualElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        __FIVectorView_1_Windows__CUI__CComposition__CIVisualElement** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CComposition__CIVisualElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CComposition__CIVisualElementVtbl;

interface __FIVector_1_Windows__CUI__CComposition__CIVisualElement
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CComposition__CIVisualElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CComposition__CIVisualElement_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CComposition__CIVisualElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CUI__CColor __FIReference_1_Windows__CUI__CColor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CUI__CColor;

typedef struct __FIReference_1_Windows__CUI__CColorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CUI__CColor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CUI__CColor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CUI__CColor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CUI__CColor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CUI__CColor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CUI__CColor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CUI__CColor* This,
        struct __x_ABI_CWindows_CUI_CColor* result);

    END_INTERFACE
} __FIReference_1_Windows__CUI__CColorVtbl;

interface __FIReference_1_Windows__CUI__CColor
{
    CONST_VTBL struct __FIReference_1_Windows__CUI__CColorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CUI__CColor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CUI__CColor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CUI__CColor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CUI__CColor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CUI__CColor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CUI__CColor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CUI__CColor_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CUI__CColor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* sender,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* sender,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* sender,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* sender,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContentRoot __x_ABI_CWindows_CUI_CIUIContentRoot;

#endif // ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowClosedReason __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowClosedReason;

typedef enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowFrameStyle __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowFrameStyle;

typedef enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind;

typedef enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowTitleBarVisibility __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowTitleBarVisibility;

typedef enum __x_ABI_CWindows_CUI_CWindowManagement_CWindowingEnvironmentKind __x_ABI_CWindows_CUI_CWindowManagement_CWindowingEnvironmentKind;

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowClosedReason
{
    AppWindowClosedReason_Other = 0,
    AppWindowClosedReason_AppInitiated = 1,
    AppWindowClosedReason_UserInitiated = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowFrameStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowFrameStyle
{
    AppWindowFrameStyle_Default = 0,
    AppWindowFrameStyle_NoFrame = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowPresentationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind
{
    AppWindowPresentationKind_Default = 0,
    AppWindowPresentationKind_CompactOverlay = 1,
    AppWindowPresentationKind_FullScreen = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.AppWindowTitleBarVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowTitleBarVisibility
{
    AppWindowTitleBarVisibility_Default = 0,
    AppWindowTitleBarVisibility_AlwaysHidden = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.UI.WindowManagement.WindowingEnvironmentKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CUI_CWindowManagement_CWindowingEnvironmentKind
{
    WindowingEnvironmentKind_Unknown = 0,
    WindowingEnvironmentKind_Overlapped = 1,
    WindowingEnvironmentKind_Tiled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindow[] = L"Windows.UI.WindowManagement.IAppWindow";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CIUIContentRoot** value);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);
    HRESULT (STDMETHODCALLTYPE* get_Frame)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PersistedStateId)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PersistedStateId)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Presenter)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter** value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_TitleBar)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar** value);
    HRESULT (STDMETHODCALLTYPE* get_UIContext)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CIUIContext** value);
    HRESULT (STDMETHODCALLTYPE* get_WindowingEnvironment)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** value);
    HRESULT (STDMETHODCALLTYPE* CloseAsync)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetPlacement)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement** result);
    HRESULT (STDMETHODCALLTYPE* GetDisplayRegions)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result);
    HRESULT (STDMETHODCALLTYPE* RequestMoveToDisplayRegion)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* displayRegion);
    HRESULT (STDMETHODCALLTYPE* RequestMoveAdjacentToCurrentView)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This);
    HRESULT (STDMETHODCALLTYPE* RequestMoveAdjacentToWindow)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* anchorWindow);
    HRESULT (STDMETHODCALLTYPE* RequestMoveRelativeToWindowContent)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* anchorWindow,
        struct __x_ABI_CWindows_CFoundation_CPoint contentOffset);
    HRESULT (STDMETHODCALLTYPE* RequestMoveRelativeToCurrentViewContent)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        struct __x_ABI_CWindows_CFoundation_CPoint contentOffset);
    HRESULT (STDMETHODCALLTYPE* RequestMoveRelativeToDisplayRegion)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* displayRegion,
        struct __x_ABI_CWindows_CFoundation_CPoint displayRegionOffset);
    HRESULT (STDMETHODCALLTYPE* RequestSize)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        struct __x_ABI_CWindows_CFoundation_CSize frameSize);
    HRESULT (STDMETHODCALLTYPE* TryShowAsync)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowClosedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CloseRequested)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CAppWindow_Windows__CUI__CWindowManagement__CAppWindowCloseRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CloseRequested)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_Frame(This, value) \
    ((This)->lpVtbl->get_Frame(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_PersistedStateId(This, value) \
    ((This)->lpVtbl->get_PersistedStateId(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_put_PersistedStateId(This, value) \
    ((This)->lpVtbl->put_PersistedStateId(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_Presenter(This, value) \
    ((This)->lpVtbl->get_Presenter(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_TitleBar(This, value) \
    ((This)->lpVtbl->get_TitleBar(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_UIContext(This, value) \
    ((This)->lpVtbl->get_UIContext(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_get_WindowingEnvironment(This, value) \
    ((This)->lpVtbl->get_WindowingEnvironment(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_CloseAsync(This, operation) \
    ((This)->lpVtbl->CloseAsync(This, operation))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_GetPlacement(This, result) \
    ((This)->lpVtbl->GetPlacement(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_GetDisplayRegions(This, result) \
    ((This)->lpVtbl->GetDisplayRegions(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveToDisplayRegion(This, displayRegion) \
    ((This)->lpVtbl->RequestMoveToDisplayRegion(This, displayRegion))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveAdjacentToCurrentView(This) \
    ((This)->lpVtbl->RequestMoveAdjacentToCurrentView(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveAdjacentToWindow(This, anchorWindow) \
    ((This)->lpVtbl->RequestMoveAdjacentToWindow(This, anchorWindow))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveRelativeToWindowContent(This, anchorWindow, contentOffset) \
    ((This)->lpVtbl->RequestMoveRelativeToWindowContent(This, anchorWindow, contentOffset))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveRelativeToCurrentViewContent(This, contentOffset) \
    ((This)->lpVtbl->RequestMoveRelativeToCurrentViewContent(This, contentOffset))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestMoveRelativeToDisplayRegion(This, displayRegion, displayRegionOffset) \
    ((This)->lpVtbl->RequestMoveRelativeToDisplayRegion(This, displayRegion, displayRegionOffset))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_RequestSize(This, frameSize) \
    ((This)->lpVtbl->RequestSize(This, frameSize))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_TryShowAsync(This, operation) \
    ((This)->lpVtbl->TryShowAsync(This, operation))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_add_Closed(This, handler, token) \
    ((This)->lpVtbl->add_Closed(This, handler, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_add_CloseRequested(This, handler, token) \
    ((This)->lpVtbl->add_CloseRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_remove_CloseRequested(This, token) \
    ((This)->lpVtbl->remove_CloseRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowChangedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DidAvailableWindowPresentationsChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidDisplayRegionsChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidFrameChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidSizeChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidTitleBarChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidVisibilityChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidWindowingEnvironmentChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DidWindowPresentationChange)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidAvailableWindowPresentationsChange(This, value) \
    ((This)->lpVtbl->get_DidAvailableWindowPresentationsChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidDisplayRegionsChange(This, value) \
    ((This)->lpVtbl->get_DidDisplayRegionsChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidFrameChange(This, value) \
    ((This)->lpVtbl->get_DidFrameChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidSizeChange(This, value) \
    ((This)->lpVtbl->get_DidSizeChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidTitleBarChange(This, value) \
    ((This)->lpVtbl->get_DidTitleBarChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidVisibilityChange(This, value) \
    ((This)->lpVtbl->get_DidVisibilityChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidWindowingEnvironmentChange(This, value) \
    ((This)->lpVtbl->get_DidWindowingEnvironmentChange(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_get_DidWindowPresentationChange(This, value) \
    ((This)->lpVtbl->get_DidWindowPresentationChange(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowCloseRequestedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Cancel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Cancel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_get_Cancel(This, value) \
    ((This)->lpVtbl->get_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_put_Cancel(This, value) \
    ((This)->lpVtbl->put_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowCloseRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowClosedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowClosedEventArgs[] = L"Windows.UI.WindowManagement.IAppWindowClosedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowClosedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowClosedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowFrame[] = L"Windows.UI.WindowManagement.IAppWindowFrame";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DragRegionVisuals)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame* This,
        __FIVector_1_Windows__CUI__CComposition__CIVisualElement** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_get_DragRegionVisuals(This, value) \
    ((This)->lpVtbl->get_DragRegionVisuals(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowFrameStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowFrameStyle[] = L"Windows.UI.WindowManagement.IAppWindowFrameStyle";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFrameStyle)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowFrameStyle* result);
    HRESULT (STDMETHODCALLTYPE* SetFrameStyle)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowFrameStyle frameStyle);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyleVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_GetFrameStyle(This, result) \
    ((This)->lpVtbl->GetFrameStyle(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_SetFrameStyle(This, frameStyle) \
    ((This)->lpVtbl->SetFrameStyle(This, frameStyle))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowFrameStyle_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPlacement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPlacement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPlacement[] = L"Windows.UI.WindowManagement.IAppWindowPlacement";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayRegion)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion** value);
    HRESULT (STDMETHODCALLTYPE* get_Offset)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacementVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_get_DisplayRegion(This, value) \
    ((This)->lpVtbl->get_DisplayRegion(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_get_Offset(This, value) \
    ((This)->lpVtbl->get_Offset(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPlacement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresentationConfiguration[] = L"Windows.UI.WindowManagement.IAppWindowPresentationConfiguration";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresentationConfigurationFactory[] = L"Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfigurationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowPresenter[] = L"Windows.UI.WindowManagement.IAppWindowPresenter";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetConfiguration)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* IsPresentationSupported)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind presentationKind,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* RequestPresentation)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresentationConfiguration* configuration,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* RequestPresentationByKind)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowPresentationKind presentationKind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenterVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_GetConfiguration(This, result) \
    ((This)->lpVtbl->GetConfiguration(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_IsPresentationSupported(This, presentationKind, result) \
    ((This)->lpVtbl->IsPresentationSupported(This, presentationKind, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_RequestPresentation(This, configuration, result) \
    ((This)->lpVtbl->RequestPresentation(This, configuration, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_RequestPresentationByKind(This, presentationKind, result) \
    ((This)->lpVtbl->RequestPresentationByKind(This, presentationKind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowStatics[] = L"Windows.UI.WindowManagement.IAppWindowStatics";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryCreateAsync)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        __FIAsyncOperation_1_Windows__CUI__CWindowManagement__CAppWindow** operation);
    HRESULT (STDMETHODCALLTYPE* ClearAllPersistedState)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This);
    HRESULT (STDMETHODCALLTYPE* ClearPersistedState)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics* This,
        HSTRING key);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_TryCreateAsync(This, operation) \
    ((This)->lpVtbl->TryCreateAsync(This, operation))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_ClearAllPersistedState(This) \
    ((This)->lpVtbl->ClearAllPersistedState(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_ClearPersistedState(This, key) \
    ((This)->lpVtbl->ClearPersistedState(This, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBar[] = L"Windows.UI.WindowManagement.IAppWindowTitleBar";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonHoverBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonHoverBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonHoverForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonHoverForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonInactiveBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonInactiveBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonInactiveForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonInactiveForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonPressedBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonPressedBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonPressedForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonPressedForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendsContentIntoTitleBar)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ExtendsContentIntoTitleBar)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_InactiveBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_InactiveBackgroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_InactiveForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_InactiveForegroundColor)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetTitleBarOcclusions)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar* This,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CAppWindowTitleBarOcclusion** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonHoverBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonHoverBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonHoverBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonHoverBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonHoverForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonHoverForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonHoverForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonHoverForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonInactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonInactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonInactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonInactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonInactiveForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonInactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonInactiveForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonInactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonPressedBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonPressedBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonPressedBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonPressedBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ButtonPressedForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonPressedForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ButtonPressedForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonPressedForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ExtendsContentIntoTitleBar(This, value) \
    ((This)->lpVtbl->get_ExtendsContentIntoTitleBar(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ExtendsContentIntoTitleBar(This, value) \
    ((This)->lpVtbl->put_ExtendsContentIntoTitleBar(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_ForegroundColor(This, value) \
    ((This)->lpVtbl->put_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_InactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->get_InactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_InactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->put_InactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_InactiveForegroundColor(This, value) \
    ((This)->lpVtbl->get_InactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_put_InactiveForegroundColor(This, value) \
    ((This)->lpVtbl->put_InactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_GetTitleBarOcclusions(This, result) \
    ((This)->lpVtbl->GetTitleBarOcclusions(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBarOcclusion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBarOcclusion[] = L"Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OccludingRect)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusionVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_get_OccludingRect(This, value) \
    ((This)->lpVtbl->get_OccludingRect(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarOcclusion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IAppWindowTitleBarVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.AppWindowTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IAppWindowTitleBarVisibility[] = L"Windows.UI.WindowManagement.IAppWindowTitleBarVisibility";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPreferredVisibility)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowTitleBarVisibility* result);
    HRESULT (STDMETHODCALLTYPE* SetPreferredVisibility)(__x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CAppWindowTitleBarVisibility visibilityMode);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibilityVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_GetPreferredVisibility(This, result) \
    ((This)->lpVtbl->GetPreferredVisibility(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_SetPreferredVisibility(This, visibilityMode) \
    ((This)->lpVtbl->SetPreferredVisibility(This, visibilityMode))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIAppWindowTitleBarVisibility_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_ICompactOverlayPresentationConfiguration[] = L"Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CICompactOverlayPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IDefaultPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.DefaultPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IDefaultPresentationConfiguration[] = L"Windows.UI.WindowManagement.IDefaultPresentationConfiguration";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDefaultPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IDisplayRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.DisplayRegion
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IDisplayRegion[] = L"Windows.UI.WindowManagement.IDisplayRegion";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayMonitorDeviceId)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_WorkAreaOffset)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_WorkAreaSize)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_WindowingEnvironment)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** value);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CDisplayRegion_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegionVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_get_DisplayMonitorDeviceId(This, value) \
    ((This)->lpVtbl->get_DisplayMonitorDeviceId(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_get_WorkAreaOffset(This, value) \
    ((This)->lpVtbl->get_WorkAreaOffset(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_get_WorkAreaSize(This, value) \
    ((This)->lpVtbl->get_WorkAreaSize(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_get_WindowingEnvironment(This, value) \
    ((This)->lpVtbl->get_WindowingEnvironment(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IFullScreenPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.FullScreenPresentationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IFullScreenPresentationConfiguration[] = L"Windows.UI.WindowManagement.IFullScreenPresentationConfiguration";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsExclusive)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsExclusive)(__x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_get_IsExclusive(This, value) \
    ((This)->lpVtbl->get_IsExclusive(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_put_IsExclusive(This, value) \
    ((This)->lpVtbl->put_IsExclusive(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIFullScreenPresentationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowServicesStatics[] = L"Windows.UI.WindowManagement.IWindowServicesStatics";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllTopLevelWindowIds)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics* This,
        __FIVectorView_1_Windows__CUI__CWindowId** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_FindAllTopLevelWindowIds(This, result) \
    ((This)->lpVtbl->FindAllTopLevelWindowIds(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironment[] = L"Windows.UI.WindowManagement.IWindowingEnvironment";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CWindowingEnvironmentKind* value);
    HRESULT (STDMETHODCALLTYPE* GetDisplayRegions)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        __FITypedEventHandler_2_Windows__CUI__CWindowManagement__CWindowingEnvironment_Windows__CUI__CWindowManagement__CWindowingEnvironmentChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_GetDisplayRegions(This, result) \
    ((This)->lpVtbl->GetDisplayRegions(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentAddedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowingEnvironment)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_get_WindowingEnvironment(This, value) \
    ((This)->lpVtbl->get_WindowingEnvironment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentAddedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentChangedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentRemovedEventArgs[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowingEnvironment)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_get_WindowingEnvironment(This, value) \
    ((This)->lpVtbl->get_WindowingEnvironment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.WindowManagement.IWindowingEnvironmentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.WindowManagement.WindowingEnvironment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_WindowManagement_IWindowingEnvironmentStatics[] = L"Windows.UI.WindowManagement.IWindowingEnvironmentStatics";
typedef struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment** result);
    HRESULT (STDMETHODCALLTYPE* FindAllWithKind)(__x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics* This,
        enum __x_ABI_CWindows_CUI_CWindowManagement_CWindowingEnvironmentKind kind,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CWindowingEnvironment** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStaticsVtbl;

interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FindAll(This, result) \
    ((This)->lpVtbl->FindAll(This, result))

#define __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_FindAllWithKind(This, kind, result) \
    ((This)->lpVtbl->FindAllWithKind(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironmentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IAppWindowStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindow ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindow[] = L"Windows.UI.WindowManagement.AppWindow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowChangedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowCloseRequestedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowClosedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowClosedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowClosedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowClosedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowClosedEventArgs[] = L"Windows.UI.WindowManagement.AppWindowClosedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowFrameStyle
 *    Windows.UI.WindowManagement.IAppWindowFrame ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowFrame_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowFrame[] = L"Windows.UI.WindowManagement.AppWindowFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPlacement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPlacement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPlacement_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPlacement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPlacement[] = L"Windows.UI.WindowManagement.AppWindowPlacement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPresentationConfiguration[] = L"Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowPresenter ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowPresenter[] = L"Windows.UI.WindowManagement.AppWindowPresenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowTitleBarVisibility
 *    Windows.UI.WindowManagement.IAppWindowTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBar_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowTitleBar[] = L"Windows.UI.WindowManagement.AppWindowTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.AppWindowTitleBarOcclusion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_AppWindowTitleBarOcclusion[] = L"Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_CompactOverlayPresentationConfiguration[] = L"Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.DefaultPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IDefaultPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_DefaultPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_DefaultPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_DefaultPresentationConfiguration[] = L"Windows.UI.WindowManagement.DefaultPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.DisplayRegion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IDisplayRegion ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_DisplayRegion_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_DisplayRegion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_DisplayRegion[] = L"Windows.UI.WindowManagement.DisplayRegion";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.FullScreenPresentationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IFullScreenPresentationConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_FullScreenPresentationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_FullScreenPresentationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_FullScreenPresentationConfiguration[] = L"Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IWindowServicesStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowServices_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowServices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowServices[] = L"Windows.UI.WindowManagement.WindowServices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.WindowManagement.IWindowingEnvironmentStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironment ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironment_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironment[] = L"Windows.UI.WindowManagement.WindowingEnvironment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentAddedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentChangedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_WindowManagement_WindowingEnvironmentRemovedEventArgs[] = L"Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
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
#endif // __windows2Eui2Ewindowmanagement_p_h__

#endif // __windows2Eui2Ewindowmanagement_h__
