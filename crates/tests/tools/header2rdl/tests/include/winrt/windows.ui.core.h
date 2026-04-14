
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
#ifndef __windows2Eui2Ecore_h__
#define __windows2Eui2Ecore_h__
#ifndef __windows2Eui2Ecore_p_h__
#define __windows2Eui2Ecore_p_h__


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
#include "Windows.System.h"
#include "Windows.UI.h"
#include "Windows.UI.Composition.h"
#include "Windows.UI.Input.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IDispatchedHandler;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler ABI::Windows::UI::Core::IDispatchedHandler

#endif // ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IIdleDispatchedHandler;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler ABI::Windows::UI::Core::IIdleDispatchedHandler

#endif // ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IAcceleratorKeyEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs ABI::Windows::UI::Core::IAcceleratorKeyEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IAcceleratorKeyEventArgs2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2 ABI::Windows::UI::Core::IAcceleratorKeyEventArgs2

#endif // ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IAutomationProviderRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs ABI::Windows::UI::Core::IAutomationProviderRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IBackRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs ABI::Windows::UI::Core::IBackRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICharacterReceivedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs ABI::Windows::UI::Core::ICharacterReceivedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IClosestInteractiveBoundsRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs ABI::Windows::UI::Core::IClosestInteractiveBoundsRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreAcceleratorKeys;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys ABI::Windows::UI::Core::ICoreAcceleratorKeys

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreClosestInteractiveBoundsRequested;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested ABI::Windows::UI::Core::ICoreClosestInteractiveBoundsRequested

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreComponentFocusable;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable ABI::Windows::UI::Core::ICoreComponentFocusable

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreCursor;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreCursor ABI::Windows::UI::Core::ICoreCursor

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreCursorFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory ABI::Windows::UI::Core::ICoreCursorFactory

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreDispatcher2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2 ABI::Windows::UI::Core::ICoreDispatcher2

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreDispatcherWithTaskPriority;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority ABI::Windows::UI::Core::ICoreDispatcherWithTaskPriority

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreIndependentInputSourceController;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController ABI::Windows::UI::Core::ICoreIndependentInputSourceController

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreIndependentInputSourceControllerStatics;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics ABI::Windows::UI::Core::ICoreIndependentInputSourceControllerStatics

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreInputSourceBase;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase ABI::Windows::UI::Core::ICoreInputSourceBase

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreKeyboardInputSource;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource ABI::Windows::UI::Core::ICoreKeyboardInputSource

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreKeyboardInputSource2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2 ABI::Windows::UI::Core::ICoreKeyboardInputSource2

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICorePointerInputSource;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource ABI::Windows::UI::Core::ICorePointerInputSource

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICorePointerInputSource2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2 ABI::Windows::UI::Core::ICorePointerInputSource2

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICorePointerRedirector;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector ABI::Windows::UI::Core::ICorePointerRedirector

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreTouchHitTesting;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting ABI::Windows::UI::Core::ICoreTouchHitTesting

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2 ABI::Windows::UI::Core::ICoreWindow2

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow3;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3 ABI::Windows::UI::Core::ICoreWindow3

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow4;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4 ABI::Windows::UI::Core::ICoreWindow4

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindow5;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5 ABI::Windows::UI::Core::ICoreWindow5

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowDialog;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog ABI::Windows::UI::Core::ICoreWindowDialog

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowDialogFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory ABI::Windows::UI::Core::ICoreWindowDialogFactory

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs ABI::Windows::UI::Core::ICoreWindowEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowFlyout;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout ABI::Windows::UI::Core::ICoreWindowFlyout

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowFlyoutFactory;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory ABI::Windows::UI::Core::ICoreWindowFlyoutFactory

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowPopupShowingEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs ABI::Windows::UI::Core::ICoreWindowPopupShowingEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowResizeManager;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager ABI::Windows::UI::Core::ICoreWindowResizeManager

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowResizeManagerLayoutCapability;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability ABI::Windows::UI::Core::ICoreWindowResizeManagerLayoutCapability

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowResizeManagerStatics;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics ABI::Windows::UI::Core::ICoreWindowResizeManagerStatics

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowStatic;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic ABI::Windows::UI::Core::ICoreWindowStatic

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreWindowWithContext;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext ABI::Windows::UI::Core::ICoreWindowWithContext

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IIdleDispatchedHandlerArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs ABI::Windows::UI::Core::IIdleDispatchedHandlerArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IInitializeWithCoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow ABI::Windows::UI::Core::IInitializeWithCoreWindow

#endif // ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IInputEnabledEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs ABI::Windows::UI::Core::IInputEnabledEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IKeyEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs ABI::Windows::UI::Core::IKeyEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IKeyEventArgs2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2 ABI::Windows::UI::Core::IKeyEventArgs2

#endif // ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IPointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs ABI::Windows::UI::Core::IPointerEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ISystemNavigationManager;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager ABI::Windows::UI::Core::ISystemNavigationManager

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ISystemNavigationManager2;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2 ABI::Windows::UI::Core::ISystemNavigationManager2

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ISystemNavigationManagerStatics;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics ABI::Windows::UI::Core::ISystemNavigationManagerStatics

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ITouchHitTestingEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs ABI::Windows::UI::Core::ITouchHitTestingEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IVisibilityChangedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs ABI::Windows::UI::Core::IVisibilityChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IWindowActivatedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs ABI::Windows::UI::Core::IWindowActivatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IWindowSizeChangedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs ABI::Windows::UI::Core::IWindowSizeChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__

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


#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommand;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommand ABI::Windows::UI::Popups::IUICommand

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b8770535-6a4b-52b1-b578-f3cdc5007a1f"))
IAsyncOperation<ABI::Windows::UI::Popups::IUICommand*> : IAsyncOperation_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Popups::IUICommand*> __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dd33fd5b-a24d-5a44-91fe-dd6441770103"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Popups::IUICommand*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Popups::IUICommand*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PointerPoint;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPoint;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint ABI::Windows::UI::Input::IPointerPoint

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CPointerPoint_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CPointerPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("721fe01c-5ad4-5262-b078-3ab345105db8"))
IIterator<ABI::Windows::UI::Input::PointerPoint*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PointerPoint*, ABI::Windows::UI::Input::IPointerPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.PointerPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::PointerPoint*> __FIIterator_1_Windows__CUI__CInput__CPointerPoint_t;
#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CPointerPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CPointerPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CPointerPoint_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CPointerPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6f2cba6-7076-5b59-9631-f6ac32b57695"))
IIterable<ABI::Windows::UI::Input::PointerPoint*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PointerPoint*, ABI::Windows::UI::Input::IPointerPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.PointerPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::PointerPoint*> __FIIterable_1_Windows__CUI__CInput__CPointerPoint_t;
#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CPointerPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CPointerPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f45db3d3-7299-57ce-a73e-297cf0af3083"))
IIterator<ABI::Windows::UI::Popups::IUICommand*> : IIterator_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Popups::IUICommand*> __FIIterator_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e63de42b-53c3-5e07-90d3-98172d545412"))
IIterable<ABI::Windows::UI::Popups::IUICommand*> : IIterable_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Popups::IUICommand*> __FIIterable_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CPointerPoint_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CPointerPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f0f57411-7786-5174-8752-4c5e834b6da2"))
IVectorView<ABI::Windows::UI::Input::PointerPoint*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PointerPoint*, ABI::Windows::UI::Input::IPointerPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.PointerPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::PointerPoint*> __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_t;
#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CPointerPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CPointerPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ed1165e6-f377-5b00-8172-93c1bd04deb4"))
IVectorView<ABI::Windows::UI::Popups::IUICommand*> : IVectorView_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Popups::IUICommand*> __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CInput__CPointerPoint_USE
#define DEF___FIVector_1_Windows__CUI__CInput__CPointerPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dfa655cf-fde7-5048-b4bf-c909231b7edb"))
IVector<ABI::Windows::UI::Input::PointerPoint*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::PointerPoint*, ABI::Windows::UI::Input::IPointerPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Input.PointerPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Input::PointerPoint*> __FIVector_1_Windows__CUI__CInput__CPointerPoint_t;
#define __FIVector_1_Windows__CUI__CInput__CPointerPoint ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CInput__CPointerPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CInput__CPointerPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE
#define DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("105139a1-dcb8-5f65-97ef-cb1bf0b75f9d"))
IVector<ABI::Windows::UI::Popups::IUICommand*> : IVector_impl<ABI::Windows::UI::Popups::IUICommand*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Popups.IUICommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Popups::IUICommand*> __FIVector_1_Windows__CUI__CPopups__CIUICommand_t;
#define __FIVector_1_Windows__CUI__CPopups__CIUICommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CPopups__CIUICommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CPopups__CIUICommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class BackRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_USE
#define DEF___FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ca821060-002b-526d-8122-982630d7cdbe"))
IEventHandler<ABI::Windows::UI::Core::BackRequestedEventArgs*> : IEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::BackRequestedEventArgs*, ABI::Windows::UI::Core::IBackRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Windows.UI.Core.BackRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<ABI::Windows::UI::Core::BackRequestedEventArgs*> __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_t;
#define __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs ABI::Windows::Foundation::__FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CharacterReceivedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5aa4a848-86b2-506b-89ab-5eb5786420c6"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::CharacterReceivedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CharacterReceivedEventArgs*, ABI::Windows::UI::Core::ICharacterReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.CharacterReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::CharacterReceivedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindowEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1a8ac270-a777-50f7-88a1-e34e56c09449"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::CoreWindowEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindowEventArgs*, ABI::Windows::UI::Core::ICoreWindowEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.CoreWindowEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::CoreWindowEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class InputEnabledEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c9965f1c-3641-51b6-b823-048ec3628fb0"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::InputEnabledEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::InputEnabledEventArgs*, ABI::Windows::UI::Core::IInputEnabledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.InputEnabledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::InputEnabledEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class KeyEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eadffdf7-d70e-5688-906c-c2b1229ea16d"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::KeyEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::KeyEventArgs*, ABI::Windows::UI::Core::IKeyEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.KeyEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::KeyEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class PointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26aabf41-a0fd-5e66-b188-6c74182d00cd"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class TouchHitTestingEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c76e9d25-6a96-58fd-874f-ae52bd603af8"))
ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::TouchHitTestingEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::TouchHitTestingEventArgs*, ABI::Windows::UI::Core::ITouchHitTestingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.UI.Core.TouchHitTestingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::UI::Core::TouchHitTestingEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreComponentInputSource;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class ClosestInteractiveBoundsRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e2c62d42-0577-5112-9e59-eae159bf39e9"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreComponentInputSource*, ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreComponentInputSource*, ABI::Windows::UI::Core::ICoreInputSourceBase*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*, ABI::Windows::UI::Core::IClosestInteractiveBoundsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreComponentInputSource, Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreComponentInputSource*, ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreDispatcher;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class AcceleratorKeyEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("136dff0d-f7e8-5153-b31c-86390c701880"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreDispatcher*, ABI::Windows::UI::Core::AcceleratorKeyEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreDispatcher*, ABI::Windows::UI::Core::ICoreDispatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::AcceleratorKeyEventArgs*, ABI::Windows::UI::Core::IAcceleratorKeyEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreDispatcher, Windows.UI.Core.AcceleratorKeyEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreDispatcher*, ABI::Windows::UI::Core::AcceleratorKeyEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindow;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6368ae3d-52d4-5290-b936-717a9acf5bea"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class AutomationProviderRequestedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("54db5c04-81f7-5f46-9fb8-e49beec70a24"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::AutomationProviderRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::AutomationProviderRequestedEventArgs*, ABI::Windows::UI::Core::IAutomationProviderRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.AutomationProviderRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::AutomationProviderRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("73f846a8-0af6-5872-8fb8-ae2f56d8553e"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CharacterReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CharacterReceivedEventArgs*, ABI::Windows::UI::Core::ICharacterReceivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.CharacterReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CharacterReceivedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("21a652d2-bfe2-5b2e-a2ab-ca45253be8b0"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*, ABI::Windows::UI::Core::IClosestInteractiveBoundsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ClosestInteractiveBoundsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d08e4f66-3457-57f2-ba0c-cb347133bd15"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CoreWindowEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindowEventArgs*, ABI::Windows::UI::Core::ICoreWindowEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.CoreWindowEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CoreWindowEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindowPopupShowingEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b32d6422-78b2-5e00-84a8-6e3167aaabde"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CoreWindowPopupShowingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindowPopupShowingEventArgs*, ABI::Windows::UI::Core::ICoreWindowPopupShowingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.CoreWindowPopupShowingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::CoreWindowPopupShowingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_USE */

#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e230a64a-506a-59c3-bb61-5559ff995663"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::InputEnabledEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::InputEnabledEventArgs*, ABI::Windows::UI::Core::IInputEnabledEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.InputEnabledEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::InputEnabledEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a3ec0774-55ac-5d61-8232-b35c5d35c93c"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::KeyEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::KeyEventArgs*, ABI::Windows::UI::Core::IKeyEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.KeyEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::KeyEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("420e1bb6-e99d-5e64-8e25-07467e3cae9e"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("197654c9-0c47-502b-9aa1-0deb03ed9702"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::TouchHitTestingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::TouchHitTestingEventArgs*, ABI::Windows::UI::Core::ITouchHitTestingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.TouchHitTestingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::TouchHitTestingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class VisibilityChangedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("14b7f692-5583-52a1-aa42-fc1843c0f748"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::VisibilityChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::VisibilityChangedEventArgs*, ABI::Windows::UI::Core::IVisibilityChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.VisibilityChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::VisibilityChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class WindowActivatedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("26a73b35-a7f9-52db-88d6-15726deb2523"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::WindowActivatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::WindowActivatedEventArgs*, ABI::Windows::UI::Core::IWindowActivatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.WindowActivatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::WindowActivatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class WindowSizeChangedEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("318dbb67-4089-5690-9040-1d454fb2f686"))
ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::WindowSizeChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::ICoreWindow*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::WindowSizeChangedEventArgs*, ABI::Windows::UI::Core::IWindowSizeChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.CoreWindow, Windows.UI.Core.WindowSizeChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::CoreWindow*, ABI::Windows::UI::Core::WindowSizeChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("29742d27-177d-54c3-b974-616fc45a2b0c"))
ITypedEventHandler<ABI::Windows::UI::Core::ICorePointerRedirector*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::UI::Core::ICorePointerRedirector*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Core.ICorePointerRedirector, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Core::ICorePointerRedirector*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

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
        namespace System {
            typedef enum VirtualKey : int VirtualKey;
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            typedef enum VirtualKeyModifiers : unsigned int VirtualKeyModifiers;
        } /* System */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                class Visual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface IVisual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CIVisual ABI::Windows::UI::Composition::IVisual

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommandInvokedHandler;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler ABI::Windows::UI::Popups::IUICommandInvokedHandler

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

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
            namespace Core {
                typedef enum AppViewBackButtonVisibility : int AppViewBackButtonVisibility;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreAcceleratorKeyEventType : int CoreAcceleratorKeyEventType;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreCursorType : int CoreCursorType;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreDispatcherPriority : int CoreDispatcherPriority;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreIndependentInputFilters : unsigned int CoreIndependentInputFilters;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreInputDeviceTypes : unsigned int CoreInputDeviceTypes;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreProcessEventsOption : int CoreProcessEventsOption;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreVirtualKeyStates : unsigned int CoreVirtualKeyStates;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreWindowActivationMode : int CoreWindowActivationMode;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreWindowActivationState : int CoreWindowActivationState;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef enum CoreWindowFlowDirection : int CoreWindowFlowDirection;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef struct CorePhysicalKeyStatus CorePhysicalKeyStatus;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                typedef struct CoreProximityEvaluation CoreProximityEvaluation;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreCursor;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreIndependentInputSource;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreIndependentInputSourceController;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindowDialog;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindowFlyout;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreWindowResizeManager;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class IdleDispatchedHandlerArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class SystemNavigationManager;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Core.AppViewBackButtonVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum AppViewBackButtonVisibility : int
                {
                    AppViewBackButtonVisibility_Visible = 0,
                    AppViewBackButtonVisibility_Collapsed = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    AppViewBackButtonVisibility_Disabled = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreAcceleratorKeyEventType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreAcceleratorKeyEventType : int
                {
                    CoreAcceleratorKeyEventType_Character = 2,
                    CoreAcceleratorKeyEventType_DeadCharacter = 3,
                    CoreAcceleratorKeyEventType_KeyDown = 0,
                    CoreAcceleratorKeyEventType_KeyUp = 1,
                    CoreAcceleratorKeyEventType_SystemCharacter = 6,
                    CoreAcceleratorKeyEventType_SystemDeadCharacter = 7,
                    CoreAcceleratorKeyEventType_SystemKeyDown = 4,
                    CoreAcceleratorKeyEventType_SystemKeyUp = 5,
                    CoreAcceleratorKeyEventType_UnicodeCharacter = 8,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreCursorType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreCursorType : int
                {
                    CoreCursorType_Arrow = 0,
                    CoreCursorType_Cross = 1,
                    CoreCursorType_Custom = 2,
                    CoreCursorType_Hand = 3,
                    CoreCursorType_Help = 4,
                    CoreCursorType_IBeam = 5,
                    CoreCursorType_SizeAll = 6,
                    CoreCursorType_SizeNortheastSouthwest = 7,
                    CoreCursorType_SizeNorthSouth = 8,
                    CoreCursorType_SizeNorthwestSoutheast = 9,
                    CoreCursorType_SizeWestEast = 10,
                    CoreCursorType_UniversalNo = 11,
                    CoreCursorType_UpArrow = 12,
                    CoreCursorType_Wait = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    CoreCursorType_Pin = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    CoreCursorType_Person = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreDispatcherPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreDispatcherPriority : int
                {
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    CoreDispatcherPriority_Idle = -2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    CoreDispatcherPriority_Low = -1,
                    CoreDispatcherPriority_Normal = 0,
                    CoreDispatcherPriority_High = 1,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreIndependentInputFilters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreIndependentInputFilters : unsigned int
                {
                    CoreIndependentInputFilters_None = 0,
                    CoreIndependentInputFilters_MouseButton = 0x1,
                    CoreIndependentInputFilters_MouseWheel = 0x2,
                    CoreIndependentInputFilters_MouseHover = 0x4,
                    CoreIndependentInputFilters_PenWithBarrel = 0x8,
                    CoreIndependentInputFilters_PenInverted = 0x10,
                };

                DEFINE_ENUM_FLAG_OPERATORS(CoreIndependentInputFilters)
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.UI.Core.CoreInputDeviceTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreInputDeviceTypes : unsigned int
                {
                    CoreInputDeviceTypes_None = 0,
                    CoreInputDeviceTypes_Touch = 0x1,
                    CoreInputDeviceTypes_Pen = 0x2,
                    CoreInputDeviceTypes_Mouse = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(CoreInputDeviceTypes)
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProcessEventsOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreProcessEventsOption : int
                {
                    CoreProcessEventsOption_ProcessOneAndAllPending = 0,
                    CoreProcessEventsOption_ProcessOneIfPresent = 1,
                    CoreProcessEventsOption_ProcessUntilQuit = 2,
                    CoreProcessEventsOption_ProcessAllIfPresent = 3,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProximityEvaluationScore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreProximityEvaluationScore : int
                {
                    CoreProximityEvaluationScore_Closest = 0,
                    CoreProximityEvaluationScore_Farthest = 2147483647,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreVirtualKeyStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreVirtualKeyStates : unsigned int
                {
                    CoreVirtualKeyStates_None = 0,
                    CoreVirtualKeyStates_Down = 0x1,
                    CoreVirtualKeyStates_Locked = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(CoreVirtualKeyStates)
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreWindowActivationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreWindowActivationMode : int
                {
                    CoreWindowActivationMode_None = 0,
                    CoreWindowActivationMode_Deactivated = 1,
                    CoreWindowActivationMode_ActivatedNotForeground = 2,
                    CoreWindowActivationMode_ActivatedInForeground = 3,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.Core.CoreWindowActivationState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreWindowActivationState : int
                {
                    CoreWindowActivationState_CodeActivated = 0,
                    CoreWindowActivationState_Deactivated = 1,
                    CoreWindowActivationState_PointerActivated = 2,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreWindowFlowDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                enum CoreWindowFlowDirection : int
                {
                    CoreWindowFlowDirection_LeftToRight = 0,
                    CoreWindowFlowDirection_RightToLeft = 1,
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CorePhysicalKeyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                struct CorePhysicalKeyStatus
                {
                    UINT32 RepeatCount;
                    UINT32 ScanCode;
                    boolean IsExtendedKey;
                    boolean IsMenuKeyDown;
                    boolean WasKeyDown;
                    boolean IsKeyReleased;
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProximityEvaluation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                struct CoreProximityEvaluation
                {
                    INT32 Score;
                    ABI::Windows::Foundation::Point AdjustedPoint;
                };
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Core.DispatchedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("d1f276c4-98d8-4636-bf49-eb79507548e9")
                IDispatchedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDispatchedHandler = __uuidof(IDispatchedHandler);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIDispatchedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Core.IdleDispatchedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("a42b0c24-7f21-4abc-99c1-8f01007f0880")
                IIdleDispatchedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::UI::Core::IIdleDispatchedHandlerArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIdleDispatchedHandler = __uuidof(IIdleDispatchedHandler);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IAcceleratorKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAcceleratorKeyEventArgs[] = L"Windows.UI.Core.IAcceleratorKeyEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("ff1c4c4a-9287-470b-836e-9086e3126ade")
                IAcceleratorKeyEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EventType(
                        ABI::Windows::UI::Core::CoreAcceleratorKeyEventType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VirtualKey(
                        ABI::Windows::System::VirtualKey* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeyStatus(
                        ABI::Windows::UI::Core::CorePhysicalKeyStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAcceleratorKeyEventArgs = __uuidof(IAcceleratorKeyEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IAcceleratorKeyEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAcceleratorKeyEventArgs2[] = L"Windows.UI.Core.IAcceleratorKeyEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("d300a9f6-2f7e-4873-a555-166e596ee1c5")
                IAcceleratorKeyEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAcceleratorKeyEventArgs2 = __uuidof(IAcceleratorKeyEventArgs2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.IAutomationProviderRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AutomationProviderRequestedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAutomationProviderRequestedEventArgs[] = L"Windows.UI.Core.IAutomationProviderRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("961ff258-21bf-4b42-a298-fa479d4c52e2")
                IAutomationProviderRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AutomationProvider(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutomationProvider(
                        IInspectable* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAutomationProviderRequestedEventArgs = __uuidof(IAutomationProviderRequestedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IBackRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.BackRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IBackRequestedEventArgs[] = L"Windows.UI.Core.IBackRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("d603d28a-e411-4a4e-ba41-6a327a8675bc")
                IBackRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackRequestedEventArgs = __uuidof(IBackRequestedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICharacterReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CharacterReceivedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICharacterReceivedEventArgs[] = L"Windows.UI.Core.ICharacterReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("c584659f-99b2-4bcc-bd33-04e63f42902e")
                ICharacterReceivedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_KeyCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeyStatus(
                        ABI::Windows::UI::Core::CorePhysicalKeyStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICharacterReceivedEventArgs = __uuidof(ICharacterReceivedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IClosestInteractiveBoundsRequestedEventArgs[] = L"Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("347c11d7-f6f8-40e3-b29f-ae50d3e86486")
                IClosestInteractiveBoundsRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PointerPosition(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SearchBounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ClosestInteractiveBounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ClosestInteractiveBounds(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IClosestInteractiveBoundsRequestedEventArgs = __uuidof(IClosestInteractiveBoundsRequestedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreAcceleratorKeys
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreAcceleratorKeys[] = L"Windows.UI.Core.ICoreAcceleratorKeys";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("9ffdf7f5-b8c9-4ef0-b7d2-1de626561fc8")
                ICoreAcceleratorKeys : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AcceleratorKeyActivated(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AcceleratorKeyActivated(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreAcceleratorKeys = __uuidof(ICoreAcceleratorKeys);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreClosestInteractiveBoundsRequested
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreClosestInteractiveBoundsRequested[] = L"Windows.UI.Core.ICoreClosestInteractiveBoundsRequested";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("f303043a-e8bf-4e8e-ae69-c9dadd57a114")
                ICoreClosestInteractiveBoundsRequested : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ClosestInteractiveBoundsRequested(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ClosestInteractiveBoundsRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreClosestInteractiveBoundsRequested = __uuidof(ICoreClosestInteractiveBoundsRequested);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreComponentFocusable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreComponentFocusable[] = L"Windows.UI.Core.ICoreComponentFocusable";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("52f96fa3-8742-4411-ae69-79a85f29ac8b")
                ICoreComponentFocusable : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasFocus(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_GotFocus(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_GotFocus(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LostFocus(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LostFocus(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreComponentFocusable = __uuidof(ICoreComponentFocusable);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreCursor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreCursor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreCursor[] = L"Windows.UI.Core.ICoreCursor";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("96893acf-111d-442c-8a77-b87992f8e2d6")
                ICoreCursor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::UI::Core::CoreCursorType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreCursor = __uuidof(ICoreCursor);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreCursor;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreCursorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreCursor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreCursorFactory[] = L"Windows.UI.Core.ICoreCursorFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("f6359621-a79d-4ed3-8c32-a9ef9d6b76a4")
                ICoreCursorFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCursor(
                        ABI::Windows::UI::Core::CoreCursorType type,
                        UINT32 id,
                        ABI::Windows::UI::Core::ICoreCursor** cursor
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreCursorFactory = __uuidof(ICoreCursorFactory);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreCursorFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreAcceleratorKeys
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcher[] = L"Windows.UI.Core.ICoreDispatcher";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("60db2fa8-b705-4fde-a7d6-ebbb1891d39e")
                ICoreDispatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasThreadAccess(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessEvents(
                        ABI::Windows::UI::Core::CoreProcessEventsOption options
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RunAsync(
                        ABI::Windows::UI::Core::CoreDispatcherPriority priority,
                        ABI::Windows::UI::Core::IDispatchedHandler* agileCallback,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RunIdleAsync(
                        ABI::Windows::UI::Core::IIdleDispatchedHandler* agileCallback,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreDispatcher = __uuidof(ICoreDispatcher);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcher;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcher2[] = L"Windows.UI.Core.ICoreDispatcher2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("6f5e63c7-e3aa-4eae-b0e0-dcf321ca4b2f")
                ICoreDispatcher2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryRunAsync(
                        ABI::Windows::UI::Core::CoreDispatcherPriority priority,
                        ABI::Windows::UI::Core::IDispatchedHandler* agileCallback,
                        __FIAsyncOperation_1_boolean** asyncOperation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryRunIdleAsync(
                        ABI::Windows::UI::Core::IIdleDispatchedHandler* agileCallback,
                        __FIAsyncOperation_1_boolean** asyncOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreDispatcher2 = __uuidof(ICoreDispatcher2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcher2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcherWithTaskPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcherWithTaskPriority[] = L"Windows.UI.Core.ICoreDispatcherWithTaskPriority";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("bafaecad-484d-41be-ba80-1d58c65263ea")
                ICoreDispatcherWithTaskPriority : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentPriority(
                        ABI::Windows::UI::Core::CoreDispatcherPriority* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CurrentPriority(
                        ABI::Windows::UI::Core::CoreDispatcherPriority value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShouldYield(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShouldYieldToPriority(
                        ABI::Windows::UI::Core::CoreDispatcherPriority priority,
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopProcessEvents(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreDispatcherWithTaskPriority = __uuidof(ICoreDispatcherWithTaskPriority);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreIndependentInputSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreIndependentInputSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreIndependentInputSourceController[] = L"Windows.UI.Core.ICoreIndependentInputSourceController";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("0963261c-84fe-578a-83ca-6425309ccde4")
                ICoreIndependentInputSourceController : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsTransparentForUncontrolledInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsTransparentForUncontrolledInput(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPalmRejectionEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsPalmRejectionEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::UI::Core::ICoreInputSourceBase** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetControlledInput(
                        ABI::Windows::UI::Core::CoreInputDeviceTypes inputTypes
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetControlledInputWithFilters(
                        ABI::Windows::UI::Core::CoreInputDeviceTypes inputTypes,
                        ABI::Windows::UI::Core::CoreIndependentInputFilters required,
                        ABI::Windows::UI::Core::CoreIndependentInputFilters excluded
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreIndependentInputSourceController = __uuidof(ICoreIndependentInputSourceController);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Core.ICoreIndependentInputSourceControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreIndependentInputSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreIndependentInputSourceControllerStatics[] = L"Windows.UI.Core.ICoreIndependentInputSourceControllerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("3edc4e20-9a8a-5691-8586-fca4cb57526d")
                ICoreIndependentInputSourceControllerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForVisual(
                        ABI::Windows::UI::Composition::IVisual* visual,
                        ABI::Windows::UI::Core::ICoreIndependentInputSourceController** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForIVisualElement(
                        ABI::Windows::UI::Composition::IVisualElement* visualElement,
                        ABI::Windows::UI::Core::ICoreIndependentInputSourceController** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreIndependentInputSourceControllerStatics = __uuidof(ICoreIndependentInputSourceControllerStatics);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Core.ICoreInputSourceBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreInputSourceBase[] = L"Windows.UI.Core.ICoreInputSourceBase";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("9f488807-4580-4be8-be68-92a9311713bb")
                ICoreInputSourceBase : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Dispatcher(
                        ABI::Windows::UI::Core::ICoreDispatcher** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInputEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInputEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_InputEnabled(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InputEnabled(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreInputSourceBase = __uuidof(ICoreInputSourceBase);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreKeyboardInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreKeyboardInputSource[] = L"Windows.UI.Core.ICoreKeyboardInputSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("231c9088-e469-4df1-b208-6e490d71cb90")
                ICoreKeyboardInputSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentKeyState(
                        ABI::Windows::System::VirtualKey virtualKey,
                        ABI::Windows::UI::Core::CoreVirtualKeyStates* KeyState
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CharacterReceived(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CharacterReceived(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyDown(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyDown(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyUp(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyUp(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreKeyboardInputSource = __uuidof(ICoreKeyboardInputSource);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreKeyboardInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreKeyboardInputSource2[] = L"Windows.UI.Core.ICoreKeyboardInputSource2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("fa24cb94-f963-47a5-8778-207c482b0afd")
                ICoreKeyboardInputSource2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentKeyEventDeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreKeyboardInputSource2 = __uuidof(ICoreKeyboardInputSource2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICorePointerInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerInputSource[] = L"Windows.UI.Core.ICorePointerInputSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("bbf1bb18-e47a-48eb-8807-f8f8d3ea4551")
                ICorePointerInputSource : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReleasePointerCapture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPointerCapture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasCapture(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerPosition(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerCursor(
                        ABI::Windows::UI::Core::ICoreCursor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PointerCursor(
                        ABI::Windows::UI::Core::ICoreCursor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerCaptureLost(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerCaptureLost(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerEntered(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerEntered(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerExited(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerExited(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerMoved(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerMoved(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerPressed(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerPressed(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerReleased(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerReleased(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerWheelChanged(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerWheelChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICorePointerInputSource = __uuidof(ICorePointerInputSource);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICorePointerInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICorePointerInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerInputSource2[] = L"Windows.UI.Core.ICorePointerInputSource2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("d703708a-4516-4786-b1e5-2751d563f997")
                ICorePointerInputSource2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICorePointerInputSource2 = __uuidof(ICorePointerInputSource2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Core.ICorePointerRedirector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerRedirector[] = L"Windows.UI.Core.ICorePointerRedirector";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("8f9d0c94-5688-4b0c-a9f1-f931f7fa3dc3")
                ICorePointerRedirector : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_PointerRoutedAway(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerRoutedAway(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerRoutedTo(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerRoutedTo(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerRoutedReleased(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerRoutedReleased(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICorePointerRedirector = __uuidof(ICorePointerRedirector);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerRedirector;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Core.ICoreTouchHitTesting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreTouchHitTesting[] = L"Windows.UI.Core.ICoreTouchHitTesting";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("b1d8a289-3acf-4124-9fa3-ea8aba353c21")
                ICoreTouchHitTesting : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_TouchHitTesting(
                        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TouchHitTesting(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreTouchHitTesting = __uuidof(ICoreTouchHitTesting);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow[] = L"Windows.UI.Core.ICoreWindow";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("79b9d5f2-879e-4b89-b798-79e47598030c")
                ICoreWindow : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AutomationHostProvider(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomProperties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Dispatcher(
                        ABI::Windows::UI::Core::ICoreDispatcher** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FlowDirection(
                        ABI::Windows::UI::Core::CoreWindowFlowDirection* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FlowDirection(
                        ABI::Windows::UI::Core::CoreWindowFlowDirection value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInputEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInputEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerCursor(
                        ABI::Windows::UI::Core::ICoreCursor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PointerCursor(
                        ABI::Windows::UI::Core::ICoreCursor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PointerPosition(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Activate(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAsyncKeyState(
                        ABI::Windows::System::VirtualKey virtualKey,
                        ABI::Windows::UI::Core::CoreVirtualKeyStates* KeyState
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetKeyState(
                        ABI::Windows::System::VirtualKey virtualKey,
                        ABI::Windows::UI::Core::CoreVirtualKeyStates* KeyState
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReleasePointerCapture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPointerCapture(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Activated(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AutomationProviderRequested(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AutomationProviderRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CharacterReceived(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CharacterReceived(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_InputEnabled(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InputEnabled(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyDown(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyDown(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_KeyUp(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_KeyUp(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerCaptureLost(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerCaptureLost(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerEntered(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerEntered(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerExited(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerExited(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerMoved(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerMoved(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerPressed(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerPressed(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerReleased(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerReleased(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TouchHitTesting(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TouchHitTesting(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PointerWheelChanged(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PointerWheelChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SizeChanged(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SizeChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_VisibilityChanged(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VisibilityChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindow = __uuidof(ICoreWindow);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow2[] = L"Windows.UI.Core.ICoreWindow2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("7c2b1b85-6917-4361-9c02-0d9e3a420b95")
                ICoreWindow2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PointerPosition(
                        ABI::Windows::Foundation::Point value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindow2 = __uuidof(ICoreWindow2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow3[] = L"Windows.UI.Core.ICoreWindow3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("32c20dd8-faef-4375-a2ab-32640e4815c7")
                ICoreWindow3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ClosestInteractiveBoundsRequested(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ClosestInteractiveBoundsRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentKeyEventDeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindow3 = __uuidof(ICoreWindow3);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow3;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow4[] = L"Windows.UI.Core.ICoreWindow4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("35caf0d0-47f0-436c-af97-0dd88f6f5f02")
                ICoreWindow4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ResizeStarted(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ResizeStarted(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ResizeCompleted(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* handler,
                        EventRegistrationToken* pCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ResizeCompleted(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindow4 = __uuidof(ICoreWindow4);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow4;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow5[] = L"Windows.UI.Core.ICoreWindow5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("4b4ae1e1-2e6d-4eaa-bda1-1c5cc1bee141")
                ICoreWindow5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                        ABI::Windows::System::IDispatcherQueue** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivationMode(
                        ABI::Windows::UI::Core::CoreWindowActivationMode* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindow5 = __uuidof(ICoreWindow5);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow5;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowDialog
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowDialog
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowDialog[] = L"Windows.UI.Core.ICoreWindowDialog";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("e7392ce0-c78d-427e-8b2c-01ff420c69d5")
                ICoreWindowDialog : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Showing(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Showing(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInteractionDelayed(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInteractionDelayed(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultCommandIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultCommandIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CancelCommandIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CancelCommandIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackButtonCommand(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackButtonCommand(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowDialog = __uuidof(ICoreWindowDialog);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowDialog;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowDialogFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowDialog
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowDialogFactory[] = L"Windows.UI.Core.ICoreWindowDialogFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("cfb2a855-1c59-4b13-b1e5-16e29805f7c4")
                ICoreWindowDialogFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTitle(
                        HSTRING title,
                        ABI::Windows::UI::Core::ICoreWindowDialog** coreWindowDialog
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowDialogFactory = __uuidof(ICoreWindowDialogFactory);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowEventArgs[] = L"Windows.UI.Core.ICoreWindowEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("272b1ef3-c633-4da5-a26c-c6d0f56b29da")
                ICoreWindowEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Handled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Handled(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowEventArgs = __uuidof(ICoreWindowEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowFlyout
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowFlyout
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFlyout[] = L"Windows.UI.Core.ICoreWindowFlyout";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("e89d854d-2050-40bb-b344-f6f355eeb314")
                ICoreWindowFlyout : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Showing(
                        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Showing(
                        EventRegistrationToken cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInteractionDelayed(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsInteractionDelayed(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultCommandIndex(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultCommandIndex(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackButtonCommand(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackButtonCommand(
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowFlyout = __uuidof(ICoreWindowFlyout);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowFlyoutFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowFlyout
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFlyoutFactory[] = L"Windows.UI.Core.ICoreWindowFlyoutFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("dec4c6c4-93e8-4f7c-be27-cefaa1af68a7")
                ICoreWindowFlyoutFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Foundation::Point position,
                        ABI::Windows::UI::Core::ICoreWindowFlyout** coreWindowFlyout
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTitle(
                        ABI::Windows::Foundation::Point position,
                        HSTRING title,
                        ABI::Windows::UI::Core::ICoreWindowFlyout** coreWindowFlyout
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowFlyoutFactory = __uuidof(ICoreWindowFlyoutFactory);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowPopupShowingEventArgs
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowPopupShowingEventArgs
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowPopupShowingEventArgs[] = L"Windows.UI.Core.ICoreWindowPopupShowingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("26155fa2-5ba5-4ea4-a3b4-2dc7d63c8e26")
                ICoreWindowPopupShowingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetDesiredSize(
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowPopupShowingEventArgs = __uuidof(ICoreWindowPopupShowingEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManager[] = L"Windows.UI.Core.ICoreWindowResizeManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("b8f0b925-b350-48b3-a198-5c1a84700243")
                ICoreWindowResizeManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE NotifyLayoutCompleted(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowResizeManager = __uuidof(ICoreWindowResizeManager);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManagerLayoutCapability[] = L"Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("bb74f27b-a544-4301-80e6-0ae033ef4536")
                ICoreWindowResizeManagerLayoutCapability : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ShouldWaitForLayoutCompletion(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldWaitForLayoutCompletion(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowResizeManagerLayoutCapability = __uuidof(ICoreWindowResizeManagerLayoutCapability);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManagerStatics[] = L"Windows.UI.Core.ICoreWindowResizeManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("ae4a9045-6d70-49db-8e68-46ffbd17d38d")
                ICoreWindowResizeManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Core::ICoreWindowResizeManager** CoreWindowResizeManager
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowResizeManagerStatics = __uuidof(ICoreWindowResizeManagerStatics);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowStatic[] = L"Windows.UI.Core.ICoreWindowStatic";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("4d239005-3c2a-41b1-9022-536bb9cf93b1")
                ICoreWindowStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentThread(
                        ABI::Windows::UI::Core::ICoreWindow** ppWindow
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowStatic = __uuidof(ICoreWindowStatic);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowStatic;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowWithContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowWithContext[] = L"Windows.UI.Core.ICoreWindowWithContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("9ac40241-3575-4c3b-af66-e8c529d4d06c")
                ICoreWindowWithContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UIContext(
                        ABI::Windows::UI::IUIContext** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICoreWindowWithContext = __uuidof(ICoreWindowWithContext);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.IIdleDispatchedHandlerArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.IdleDispatchedHandlerArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IIdleDispatchedHandlerArgs[] = L"Windows.UI.Core.IIdleDispatchedHandlerArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("98bb6a24-dc1c-43cb-b4ed-d1c0eb2391f3")
                IIdleDispatchedHandlerArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsDispatcherIdle(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIdleDispatchedHandlerArgs = __uuidof(IIdleDispatchedHandlerArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IInitializeWithCoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IInitializeWithCoreWindow[] = L"Windows.UI.Core.IInitializeWithCoreWindow";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("188f20d6-9873-464a-ace5-57e010f465e6")
                IInitializeWithCoreWindow : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Initialize(
                        ABI::Windows::UI::Core::ICoreWindow* window
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInitializeWithCoreWindow = __uuidof(IInitializeWithCoreWindow);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IInputEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.InputEnabledEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IInputEnabledEventArgs[] = L"Windows.UI.Core.IInputEnabledEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("80371d4f-2fd8-4c24-aa86-3163a87b4e5a")
                IInputEnabledEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InputEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputEnabledEventArgs = __uuidof(IInputEnabledEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.KeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IKeyEventArgs[] = L"Windows.UI.Core.IKeyEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("5ff5e930-2544-4a17-bd78-1f2fdebb106b")
                IKeyEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VirtualKey(
                        ABI::Windows::System::VirtualKey* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeyStatus(
                        ABI::Windows::UI::Core::CorePhysicalKeyStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyEventArgs = __uuidof(IKeyEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIKeyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IKeyEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.KeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IKeyEventArgs2[] = L"Windows.UI.Core.IKeyEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("583add98-0790-4571-9b12-645ef9d79e42")
                IKeyEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKeyEventArgs2 = __uuidof(IKeyEventArgs2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.IPointerEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.PointerEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IPointerEventArgs[] = L"Windows.UI.Core.IPointerEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("920d9cb1-a5fc-4a21-8c09-49dfe6ffe25f")
                IPointerEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentPoint(
                        ABI::Windows::UI::Input::IPointerPoint** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeyModifiers(
                        ABI::Windows::System::VirtualKeyModifiers* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetIntermediatePoints(
                        __FIVector_1_Windows__CUI__CInput__CPointerPoint** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPointerEventArgs = __uuidof(IPointerEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManager[] = L"Windows.UI.Core.ISystemNavigationManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("93023118-cf50-42a6-9706-69107fa122e1")
                ISystemNavigationManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_BackRequested(
                        __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BackRequested(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemNavigationManager = __uuidof(ISystemNavigationManager);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManager2[] = L"Windows.UI.Core.ISystemNavigationManager2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("8c510401-67be-49ae-9509-671c1e54a389")
                ISystemNavigationManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppViewBackButtonVisibility(
                        ABI::Windows::UI::Core::AppViewBackButtonVisibility* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AppViewBackButtonVisibility(
                        ABI::Windows::UI::Core::AppViewBackButtonVisibility value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemNavigationManager2 = __uuidof(ISystemNavigationManager2);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManagerStatics[] = L"Windows.UI.Core.ISystemNavigationManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("dc52b5ce-bee0-4305-8c54-68228ed683b5")
                ISystemNavigationManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::Core::ISystemNavigationManager** loader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemNavigationManagerStatics = __uuidof(ISystemNavigationManagerStatics);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ITouchHitTestingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.TouchHitTestingEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ITouchHitTestingEventArgs[] = L"Windows.UI.Core.ITouchHitTestingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("22f3b823-0b7c-424e-9df7-33d4f962931b")
                ITouchHitTestingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProximityEvaluation(
                        ABI::Windows::UI::Core::CoreProximityEvaluation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProximityEvaluation(
                        ABI::Windows::UI::Core::CoreProximityEvaluation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Point(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BoundingBox(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EvaluateProximityToRect(
                        ABI::Windows::Foundation::Rect controlBoundingBox,
                        ABI::Windows::UI::Core::CoreProximityEvaluation* proximityEvaluation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EvaluateProximityToPolygon(
                        UINT32 controlVerticesLength,
                        ABI::Windows::Foundation::Point* controlVertices,
                        ABI::Windows::UI::Core::CoreProximityEvaluation* proximityEvaluation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITouchHitTestingEventArgs = __uuidof(ITouchHitTestingEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IVisibilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.VisibilityChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IVisibilityChangedEventArgs[] = L"Windows.UI.Core.IVisibilityChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("bf9918ea-d801-4564-a495-b1e84f8ad085")
                IVisibilityChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVisibilityChangedEventArgs = __uuidof(IVisibilityChangedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IWindowActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.WindowActivatedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IWindowActivatedEventArgs[] = L"Windows.UI.Core.IWindowActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("179d65e7-4658-4cb6-aa13-41d094ea255e")
                IWindowActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowActivationState(
                        ABI::Windows::UI::Core::CoreWindowActivationState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowActivatedEventArgs = __uuidof(IWindowActivatedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IWindowSizeChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.WindowSizeChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IWindowSizeChangedEventArgs[] = L"Windows.UI.Core.IWindowSizeChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                MIDL_INTERFACE("5a200ec7-0426-47dc-b86c-6f475915e451")
                IWindowSizeChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowSizeChangedEventArgs = __uuidof(IWindowSizeChangedEventArgs);
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IAcceleratorKeyEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *    Windows.UI.Core.IAcceleratorKeyEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_AcceleratorKeyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_AcceleratorKeyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_AcceleratorKeyEventArgs[] = L"Windows.UI.Core.AcceleratorKeyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.AutomationProviderRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IAutomationProviderRequestedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_AutomationProviderRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_AutomationProviderRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_AutomationProviderRequestedEventArgs[] = L"Windows.UI.Core.AutomationProviderRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.BackRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IBackRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_BackRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_BackRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_BackRequestedEventArgs[] = L"Windows.UI.Core.BackRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CharacterReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICharacterReceivedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CharacterReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CharacterReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CharacterReceivedEventArgs[] = L"Windows.UI.Core.CharacterReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs[] = L"Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Core.CoreAcceleratorKeys
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreAcceleratorKeys ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreAcceleratorKeys_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreAcceleratorKeys_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreAcceleratorKeys[] = L"Windows.UI.Core.CoreAcceleratorKeys";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreComponentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreInputSourceBase ** Default Interface **
 *    Windows.UI.Core.ICorePointerInputSource
 *    Windows.UI.Core.ICoreKeyboardInputSource
 *    Windows.UI.Core.ICoreComponentFocusable
 *    Windows.UI.Core.ICoreTouchHitTesting
 *    Windows.UI.Core.ICoreClosestInteractiveBoundsRequested
 *    Windows.UI.Core.ICoreKeyboardInputSource2
 *    Windows.UI.Core.ICorePointerInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreComponentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreComponentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreComponentInputSource[] = L"Windows.UI.Core.CoreComponentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreCursor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Core.ICoreCursorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreCursor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreCursor_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreCursor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreCursor[] = L"Windows.UI.Core.CoreCursor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreDispatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreDispatcher ** Default Interface **
 *    Windows.UI.Core.ICoreAcceleratorKeys
 *    Windows.UI.Core.ICoreDispatcherWithTaskPriority
 *    Windows.UI.Core.ICoreDispatcher2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreDispatcher_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreDispatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreDispatcher[] = L"Windows.UI.Core.CoreDispatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreInputSourceBase ** Default Interface **
 *    Windows.UI.Core.ICorePointerInputSource
 *    Windows.UI.Core.ICorePointerInputSource2
 *    Windows.UI.Core.ICorePointerRedirector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreIndependentInputSource[] = L"Windows.UI.Core.CoreIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreIndependentInputSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreIndependentInputSourceControllerStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreIndependentInputSourceController ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSourceController_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSourceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreIndependentInputSourceController[] = L"Windows.UI.Core.CoreIndependentInputSourceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Core.CoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreWindowStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindow ** Default Interface **
 *    Windows.UI.Core.ICoreWindow2
 *    Windows.UI.Core.ICorePointerRedirector
 *    Windows.UI.Core.ICoreWindow3
 *    Windows.UI.Core.ICoreWindow4
 *    Windows.UI.Core.ICoreWindow5
 *    Windows.UI.Core.ICoreWindowWithContext
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindow[] = L"Windows.UI.Core.CoreWindow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowDialog
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *   Type can be activated via the Windows.UI.Core.ICoreWindowDialogFactory interface starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowDialog ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowDialog_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowDialog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowDialog[] = L"Windows.UI.Core.CoreWindowDialog";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowEventArgs[] = L"Windows.UI.Core.CoreWindowEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowFlyout
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Core.ICoreWindowFlyoutFactory interface starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFlyout ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowFlyout_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowFlyout_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowFlyout[] = L"Windows.UI.Core.CoreWindowFlyout";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowPopupShowingEventArgs
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowPopupShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowPopupShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowPopupShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowPopupShowingEventArgs[] = L"Windows.UI.Core.CoreWindowPopupShowingEventArgs";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowResizeManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreWindowResizeManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowResizeManager ** Default Interface **
 *    Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowResizeManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowResizeManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowResizeManager[] = L"Windows.UI.Core.CoreWindowResizeManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.IdleDispatchedHandlerArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IIdleDispatchedHandlerArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_IdleDispatchedHandlerArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_IdleDispatchedHandlerArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_IdleDispatchedHandlerArgs[] = L"Windows.UI.Core.IdleDispatchedHandlerArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.InputEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IInputEnabledEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_InputEnabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_InputEnabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_InputEnabledEventArgs[] = L"Windows.UI.Core.InputEnabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.KeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IKeyEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *    Windows.UI.Core.IKeyEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_KeyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_KeyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_KeyEventArgs[] = L"Windows.UI.Core.KeyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.PointerEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IPointerEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_PointerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_PointerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_PointerEventArgs[] = L"Windows.UI.Core.PointerEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.SystemNavigationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ISystemNavigationManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ISystemNavigationManager ** Default Interface **
 *    Windows.UI.Core.ISystemNavigationManager2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_SystemNavigationManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_SystemNavigationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_SystemNavigationManager[] = L"Windows.UI.Core.SystemNavigationManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.TouchHitTestingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ITouchHitTestingEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_TouchHitTestingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_TouchHitTestingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_TouchHitTestingEventArgs[] = L"Windows.UI.Core.TouchHitTestingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.VisibilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IVisibilityChangedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_VisibilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_VisibilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_VisibilityChangedEventArgs[] = L"Windows.UI.Core.VisibilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.WindowActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IWindowActivatedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_WindowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_WindowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_WindowActivatedEventArgs[] = L"Windows.UI.Core.WindowActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.WindowSizeChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IWindowSizeChangedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_WindowSizeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_WindowSizeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_WindowSizeChangedEventArgs[] = L"Windows.UI.Core.WindowSizeChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler;

#endif // ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler;

#endif // ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2 __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreCursor __x_ABI_CWindows_CUI_CCore_CICoreCursor;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher __x_ABI_CWindows_CUI_CCore_CICoreDispatcher;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2 __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2 __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource;

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2 __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2;

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector;

#endif // ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow __x_ABI_CWindows_CUI_CCore_CICoreWindow;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow2 __x_ABI_CWindows_CUI_CCore_CICoreWindow2;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow3 __x_ABI_CWindows_CUI_CCore_CICoreWindow3;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow4 __x_ABI_CWindows_CUI_CCore_CICoreWindow4;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow5 __x_ABI_CWindows_CUI_CCore_CICoreWindow5;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow;

#endif // ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2 __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager;

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2 __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2;

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommand __x_ABI_CWindows_CUI_CPopups_CIUICommand;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPoint __x_ABI_CWindows_CUI_CInput_CIPointerPoint;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CPointerPoint __FIIterator_1_Windows__CUI__CInput__CPointerPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CPointerPoint;

typedef struct __FIIterator_1_Windows__CUI__CInput__CPointerPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CPointerPointVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CPointerPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CPointerPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CPointerPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CPointerPoint __FIIterable_1_Windows__CUI__CInput__CPointerPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CPointerPoint;

typedef struct __FIIterable_1_Windows__CUI__CInput__CPointerPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CPointerPoint* This,
        __FIIterator_1_Windows__CUI__CInput__CPointerPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CPointerPointVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CPointerPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CPointerPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CPointerPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CPopups__CIUICommand __FIIterator_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIIterator_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CPopups__CIUICommand __FIIterable_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIIterator_1_Windows__CUI__CPopups__CIUICommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIIterable_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CPopups__CIUICommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CPointerPoint __FIVectorView_1_Windows__CUI__CInput__CPointerPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CPointerPoint;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CPointerPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CPointerPointVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CPointerPoint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CPointerPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CPointerPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CPopups__CIUICommand __FIVectorView_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIVectorView_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CInput__CPointerPoint __FIVector_1_Windows__CUI__CInput__CPointerPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CInput__CPointerPoint;

typedef struct __FIVector_1_Windows__CUI__CInput__CPointerPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        __FIVectorView_1_Windows__CUI__CInput__CPointerPoint** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CInput__CPointerPoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CInput__CPointerPointVtbl;

interface __FIVector_1_Windows__CUI__CInput__CPointerPoint
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CInput__CPointerPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CInput__CPointerPoint_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CInput__CPointerPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CPopups__CIUICommand __FIVector_1_Windows__CUI__CPopups__CIUICommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CPopups__CIUICommand;

typedef struct __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __FIVectorView_1_Windows__CUI__CPopups__CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CPopups__CIUICommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl;

interface __FIVector_1_Windows__CUI__CPopups__CIUICommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CPopups__CIUICommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CPopups__CIUICommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CPopups__CIUICommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs;

typedef struct __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* args);

    END_INTERFACE
} __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgsVtbl;

interface __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs
{
    CONST_VTBL struct __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* sender,
        __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreDispatcher* sender,
        __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* sender,
        __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CVirtualKey __x_ABI_CWindows_CSystem_CVirtualKey;

typedef enum __x_ABI_CWindows_CSystem_CVirtualKeyModifiers __x_ABI_CWindows_CSystem_CVirtualKeyModifiers;

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisualElement __x_ABI_CWindows_CUI_CComposition_CIVisualElement;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisualElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisual __x_ABI_CWindows_CUI_CComposition_CIVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CCore_CAppViewBackButtonVisibility __x_ABI_CWindows_CUI_CCore_CAppViewBackButtonVisibility;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreAcceleratorKeyEventType __x_ABI_CWindows_CUI_CCore_CCoreAcceleratorKeyEventType;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType __x_ABI_CWindows_CUI_CCore_CCoreCursorType;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreIndependentInputFilters __x_ABI_CWindows_CUI_CCore_CCoreIndependentInputFilters;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreProcessEventsOption __x_ABI_CWindows_CUI_CCore_CCoreProcessEventsOption;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationMode __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationMode;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationState __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationState;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreWindowFlowDirection __x_ABI_CWindows_CUI_CCore_CCoreWindowFlowDirection;

typedef struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus;

typedef struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation;

/*
 *
 * Struct Windows.UI.Core.AppViewBackButtonVisibility
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CAppViewBackButtonVisibility
{
    AppViewBackButtonVisibility_Visible = 0,
    AppViewBackButtonVisibility_Collapsed = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    AppViewBackButtonVisibility_Disabled = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreAcceleratorKeyEventType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreAcceleratorKeyEventType
{
    CoreAcceleratorKeyEventType_Character = 2,
    CoreAcceleratorKeyEventType_DeadCharacter = 3,
    CoreAcceleratorKeyEventType_KeyDown = 0,
    CoreAcceleratorKeyEventType_KeyUp = 1,
    CoreAcceleratorKeyEventType_SystemCharacter = 6,
    CoreAcceleratorKeyEventType_SystemDeadCharacter = 7,
    CoreAcceleratorKeyEventType_SystemKeyDown = 4,
    CoreAcceleratorKeyEventType_SystemKeyUp = 5,
    CoreAcceleratorKeyEventType_UnicodeCharacter = 8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreCursorType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType
{
    CoreCursorType_Arrow = 0,
    CoreCursorType_Cross = 1,
    CoreCursorType_Custom = 2,
    CoreCursorType_Hand = 3,
    CoreCursorType_Help = 4,
    CoreCursorType_IBeam = 5,
    CoreCursorType_SizeAll = 6,
    CoreCursorType_SizeNortheastSouthwest = 7,
    CoreCursorType_SizeNorthSouth = 8,
    CoreCursorType_SizeNorthwestSoutheast = 9,
    CoreCursorType_SizeWestEast = 10,
    CoreCursorType_UniversalNo = 11,
    CoreCursorType_UpArrow = 12,
    CoreCursorType_Wait = 13,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    CoreCursorType_Pin = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    CoreCursorType_Person = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreDispatcherPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority
{
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    CoreDispatcherPriority_Idle = -2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    CoreDispatcherPriority_Low = -1,
    CoreDispatcherPriority_Normal = 0,
    CoreDispatcherPriority_High = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreIndependentInputFilters
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CUI_CCore_CCoreIndependentInputFilters
{
    CoreIndependentInputFilters_None = 0,
    CoreIndependentInputFilters_MouseButton = 0x1,
    CoreIndependentInputFilters_MouseWheel = 0x2,
    CoreIndependentInputFilters_MouseHover = 0x4,
    CoreIndependentInputFilters_PenWithBarrel = 0x8,
    CoreIndependentInputFilters_PenInverted = 0x10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.UI.Core.CoreInputDeviceTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes
{
    CoreInputDeviceTypes_None = 0,
    CoreInputDeviceTypes_Touch = 0x1,
    CoreInputDeviceTypes_Pen = 0x2,
    CoreInputDeviceTypes_Mouse = 0x4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProcessEventsOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreProcessEventsOption
{
    CoreProcessEventsOption_ProcessOneAndAllPending = 0,
    CoreProcessEventsOption_ProcessOneIfPresent = 1,
    CoreProcessEventsOption_ProcessUntilQuit = 2,
    CoreProcessEventsOption_ProcessAllIfPresent = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProximityEvaluationScore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluationScore
{
    CoreProximityEvaluationScore_Closest = 0,
    CoreProximityEvaluationScore_Farthest = 2147483647,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreVirtualKeyStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates
{
    CoreVirtualKeyStates_None = 0,
    CoreVirtualKeyStates_Down = 0x1,
    CoreVirtualKeyStates_Locked = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreWindowActivationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationMode
{
    CoreWindowActivationMode_None = 0,
    CoreWindowActivationMode_Deactivated = 1,
    CoreWindowActivationMode_ActivatedNotForeground = 2,
    CoreWindowActivationMode_ActivatedInForeground = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.UI.Core.CoreWindowActivationState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationState
{
    CoreWindowActivationState_CodeActivated = 0,
    CoreWindowActivationState_Deactivated = 1,
    CoreWindowActivationState_PointerActivated = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreWindowFlowDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CCore_CCoreWindowFlowDirection
{
    CoreWindowFlowDirection_LeftToRight = 0,
    CoreWindowFlowDirection_RightToLeft = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CorePhysicalKeyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus
{
    UINT32 RepeatCount;
    UINT32 ScanCode;
    boolean IsExtendedKey;
    boolean IsMenuKeyDown;
    boolean WasKeyDown;
    boolean IsKeyReleased;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Core.CoreProximityEvaluation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation
{
    INT32 Score;
    struct __x_ABI_CWindows_CFoundation_CPoint AdjustedPoint;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Core.DispatchedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CCore_CIDispatchedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIDispatchedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIDispatchedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_Invoke(This) \
    ((This)->lpVtbl->Invoke(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIDispatchedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIDispatchedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Core.IdleDispatchedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* This,
        __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_Invoke(This, e) \
    ((This)->lpVtbl->Invoke(This, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IAcceleratorKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAcceleratorKeyEventArgs[] = L"Windows.UI.Core.IAcceleratorKeyEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EventType)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreAcceleratorKeyEventType* value);
    HRESULT (STDMETHODCALLTYPE* get_VirtualKey)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStatus)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_get_EventType(This, value) \
    ((This)->lpVtbl->get_EventType(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_get_VirtualKey(This, value) \
    ((This)->lpVtbl->get_VirtualKey(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_get_KeyStatus(This, value) \
    ((This)->lpVtbl->get_KeyStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IAcceleratorKeyEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAcceleratorKeyEventArgs2[] = L"Windows.UI.Core.IAcceleratorKeyEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAcceleratorKeyEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.IAutomationProviderRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.AutomationProviderRequestedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IAutomationProviderRequestedEventArgs[] = L"Windows.UI.Core.IAutomationProviderRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutomationProvider)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_AutomationProvider)(__x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs* This,
        IInspectable* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_get_AutomationProvider(This, value) \
    ((This)->lpVtbl->get_AutomationProvider(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_put_AutomationProvider(This, value) \
    ((This)->lpVtbl->put_AutomationProvider(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIAutomationProviderRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IBackRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.BackRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IBackRequestedEventArgs[] = L"Windows.UI.Core.IBackRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIBackRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICharacterReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CharacterReceivedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICharacterReceivedEventArgs[] = L"Windows.UI.Core.ICharacterReceivedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyCode)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStatus)(__x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_get_KeyCode(This, value) \
    ((This)->lpVtbl->get_KeyCode(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_get_KeyStatus(This, value) \
    ((This)->lpVtbl->get_KeyStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICharacterReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IClosestInteractiveBoundsRequestedEventArgs[] = L"Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerPosition)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_SearchBounds)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_ClosestInteractiveBounds)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_ClosestInteractiveBounds)(__x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_get_PointerPosition(This, value) \
    ((This)->lpVtbl->get_PointerPosition(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_get_SearchBounds(This, value) \
    ((This)->lpVtbl->get_SearchBounds(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_get_ClosestInteractiveBounds(This, value) \
    ((This)->lpVtbl->get_ClosestInteractiveBounds(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_put_ClosestInteractiveBounds(This, value) \
    ((This)->lpVtbl->put_ClosestInteractiveBounds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIClosestInteractiveBoundsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreAcceleratorKeys
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreAcceleratorKeys[] = L"Windows.UI.Core.ICoreAcceleratorKeys";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeysVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AcceleratorKeyActivated)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreDispatcher_Windows__CUI__CCore__CAcceleratorKeyEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_AcceleratorKeyActivated)(__x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeysVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeysVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_add_AcceleratorKeyActivated(This, handler, pCookie) \
    ((This)->lpVtbl->add_AcceleratorKeyActivated(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_remove_AcceleratorKeyActivated(This, cookie) \
    ((This)->lpVtbl->remove_AcceleratorKeyActivated(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreAcceleratorKeys_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreClosestInteractiveBoundsRequested
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreClosestInteractiveBoundsRequested[] = L"Windows.UI.Core.ICoreClosestInteractiveBoundsRequested";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequestedVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ClosestInteractiveBoundsRequested)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreComponentInputSource_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ClosestInteractiveBoundsRequested)(__x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequestedVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequestedVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_add_ClosestInteractiveBoundsRequested(This, handler, pCookie) \
    ((This)->lpVtbl->add_ClosestInteractiveBoundsRequested(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_remove_ClosestInteractiveBoundsRequested(This, cookie) \
    ((This)->lpVtbl->remove_ClosestInteractiveBoundsRequested(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreClosestInteractiveBoundsRequested_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreComponentFocusable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreComponentFocusable[] = L"Windows.UI.Core.ICoreComponentFocusable";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasFocus)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_GotFocus)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_GotFocus)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_LostFocus)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_LostFocus)(__x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusableVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_get_HasFocus(This, value) \
    ((This)->lpVtbl->get_HasFocus(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_add_GotFocus(This, handler, pCookie) \
    ((This)->lpVtbl->add_GotFocus(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_remove_GotFocus(This, cookie) \
    ((This)->lpVtbl->remove_GotFocus(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_add_LostFocus(This, handler, pCookie) \
    ((This)->lpVtbl->add_LostFocus(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_remove_LostFocus(This, cookie) \
    ((This)->lpVtbl->remove_LostFocus(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreComponentFocusable_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreCursor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreCursor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreCursor[] = L"Windows.UI.Core.ICoreCursor";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreCursorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CUI_CCore_CICoreCursor* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreCursorVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreCursor
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreCursorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursor_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreCursor;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreCursorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreCursor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreCursorFactory[] = L"Windows.UI.Core.ICoreCursorFactory";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreCursorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCursor)(__x_ABI_CWindows_CUI_CCore_CICoreCursorFactory* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType type,
        UINT32 id,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor** cursor);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreCursorFactoryVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreCursorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_CreateCursor(This, type, id, cursor) \
    ((This)->lpVtbl->CreateCursor(This, type, id, cursor))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreCursorFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreCursorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreAcceleratorKeys
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcher[] = L"Windows.UI.Core.ICoreDispatcher";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasThreadAccess)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ProcessEvents)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreProcessEventsOption options);
    HRESULT (STDMETHODCALLTYPE* RunAsync)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority priority,
        __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* agileCallback,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* RunIdleAsync)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher* This,
        __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* agileCallback,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreDispatcherVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_get_HasThreadAccess(This, value) \
    ((This)->lpVtbl->get_HasThreadAccess(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_ProcessEvents(This, options) \
    ((This)->lpVtbl->ProcessEvents(This, options))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_RunAsync(This, priority, agileCallback, asyncAction) \
    ((This)->lpVtbl->RunAsync(This, priority, agileCallback, asyncAction))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher_RunIdleAsync(This, agileCallback, asyncAction) \
    ((This)->lpVtbl->RunIdleAsync(This, agileCallback, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcher;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcher2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcher2[] = L"Windows.UI.Core.ICoreDispatcher2";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryRunAsync)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority priority,
        __x_ABI_CWindows_CUI_CCore_CIDispatchedHandler* agileCallback,
        __FIAsyncOperation_1_boolean** asyncOperation);
    HRESULT (STDMETHODCALLTYPE* TryRunIdleAsync)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcher2* This,
        __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandler* agileCallback,
        __FIAsyncOperation_1_boolean** asyncOperation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_TryRunAsync(This, priority, agileCallback, asyncOperation) \
    ((This)->lpVtbl->TryRunAsync(This, priority, agileCallback, asyncOperation))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_TryRunIdleAsync(This, agileCallback, asyncOperation) \
    ((This)->lpVtbl->TryRunIdleAsync(This, agileCallback, asyncOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcher2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcher2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreDispatcherWithTaskPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreDispatcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreDispatcherWithTaskPriority[] = L"Windows.UI.Core.ICoreDispatcherWithTaskPriority";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriorityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPriority)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority* value);
    HRESULT (STDMETHODCALLTYPE* put_CurrentPriority)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority value);
    HRESULT (STDMETHODCALLTYPE* ShouldYield)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShouldYieldToPriority)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreDispatcherPriority priority,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* StopProcessEvents)(__x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriorityVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriorityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_get_CurrentPriority(This, value) \
    ((This)->lpVtbl->get_CurrentPriority(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_put_CurrentPriority(This, value) \
    ((This)->lpVtbl->put_CurrentPriority(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_ShouldYield(This, value) \
    ((This)->lpVtbl->ShouldYield(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_ShouldYieldToPriority(This, priority, value) \
    ((This)->lpVtbl->ShouldYieldToPriority(This, priority, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_StopProcessEvents(This) \
    ((This)->lpVtbl->StopProcessEvents(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreDispatcherWithTaskPriority_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreIndependentInputSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreIndependentInputSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreIndependentInputSourceController[] = L"Windows.UI.Core.ICoreIndependentInputSourceController";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTransparentForUncontrolledInput)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsTransparentForUncontrolledInput)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPalmRejectionEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPalmRejectionEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase** value);
    HRESULT (STDMETHODCALLTYPE* SetControlledInput)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes inputTypes);
    HRESULT (STDMETHODCALLTYPE* SetControlledInputWithFilters)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes inputTypes,
        enum __x_ABI_CWindows_CUI_CCore_CCoreIndependentInputFilters required,
        enum __x_ABI_CWindows_CUI_CCore_CCoreIndependentInputFilters excluded);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_get_IsTransparentForUncontrolledInput(This, value) \
    ((This)->lpVtbl->get_IsTransparentForUncontrolledInput(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_put_IsTransparentForUncontrolledInput(This, value) \
    ((This)->lpVtbl->put_IsTransparentForUncontrolledInput(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_get_IsPalmRejectionEnabled(This, value) \
    ((This)->lpVtbl->get_IsPalmRejectionEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_put_IsPalmRejectionEnabled(This, value) \
    ((This)->lpVtbl->put_IsPalmRejectionEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_SetControlledInput(This, inputTypes) \
    ((This)->lpVtbl->SetControlledInput(This, inputTypes))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_SetControlledInputWithFilters(This, inputTypes, required, excluded) \
    ((This)->lpVtbl->SetControlledInputWithFilters(This, inputTypes, required, excluded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Core.ICoreIndependentInputSourceControllerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreIndependentInputSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreIndependentInputSourceControllerStatics[] = L"Windows.UI.Core.ICoreIndependentInputSourceControllerStatics";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForVisual)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* visual,
        __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController** result);
    HRESULT (STDMETHODCALLTYPE* CreateForIVisualElement)(__x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisualElement* visualElement,
        __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceController** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_CreateForVisual(This, visual, result) \
    ((This)->lpVtbl->CreateForVisual(This, visual, result))

#define __x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_CreateForIVisualElement(This, visualElement, result) \
    ((This)->lpVtbl->CreateForIVisualElement(This, visualElement, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreIndependentInputSourceControllerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Core.ICoreInputSourceBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreInputSourceBase[] = L"Windows.UI.Core.ICoreInputSourceBase";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBaseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Dispatcher)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        __x_ABI_CWindows_CUI_CCore_CICoreDispatcher** value);
    HRESULT (STDMETHODCALLTYPE* get_IsInputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_InputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CInputEnabledEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_InputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBaseVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBaseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_get_Dispatcher(This, value) \
    ((This)->lpVtbl->get_Dispatcher(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_get_IsInputEnabled(This, value) \
    ((This)->lpVtbl->get_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_put_IsInputEnabled(This, value) \
    ((This)->lpVtbl->put_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_add_InputEnabled(This, handler, pCookie) \
    ((This)->lpVtbl->add_InputEnabled(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_remove_InputEnabled(This, cookie) \
    ((This)->lpVtbl->remove_InputEnabled(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreInputSourceBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreKeyboardInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreKeyboardInputSource[] = L"Windows.UI.Core.ICoreKeyboardInputSource";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentKeyState)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey virtualKey,
        enum __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates* KeyState);
    HRESULT (STDMETHODCALLTYPE* add_CharacterReceived)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CCharacterReceivedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_CharacterReceived)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_KeyDown)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_KeyDown)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_KeyUp)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_KeyUp)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSourceVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_GetCurrentKeyState(This, virtualKey, KeyState) \
    ((This)->lpVtbl->GetCurrentKeyState(This, virtualKey, KeyState))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_add_CharacterReceived(This, handler, pCookie) \
    ((This)->lpVtbl->add_CharacterReceived(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_remove_CharacterReceived(This, cookie) \
    ((This)->lpVtbl->remove_CharacterReceived(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_add_KeyDown(This, handler, pCookie) \
    ((This)->lpVtbl->add_KeyDown(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_remove_KeyDown(This, cookie) \
    ((This)->lpVtbl->remove_KeyDown(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_add_KeyUp(This, handler, pCookie) \
    ((This)->lpVtbl->add_KeyUp(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_remove_KeyUp(This, cookie) \
    ((This)->lpVtbl->remove_KeyUp(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreKeyboardInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreKeyboardInputSource2[] = L"Windows.UI.Core.ICoreKeyboardInputSource2";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentKeyEventDeviceId)(__x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_GetCurrentKeyEventDeviceId(This, value) \
    ((This)->lpVtbl->GetCurrentKeyEventDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreKeyboardInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICorePointerInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerInputSource[] = L"Windows.UI.Core.ICorePointerInputSource";
typedef struct __x_ABI_CWindows_CUI_CCore_CICorePointerInputSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReleasePointerCapture)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This);
    HRESULT (STDMETHODCALLTYPE* SetPointerCapture)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This);
    HRESULT (STDMETHODCALLTYPE* get_HasCapture)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerPosition)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerCursor)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor** value);
    HRESULT (STDMETHODCALLTYPE* put_PointerCursor)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor* value);
    HRESULT (STDMETHODCALLTYPE* add_PointerCaptureLost)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerCaptureLost)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerEntered)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerEntered)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerExited)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerExited)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerMoved)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerMoved)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerPressed)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerPressed)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerReleased)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerReleased)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerWheelChanged)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerWheelChanged)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICorePointerInputSourceVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICorePointerInputSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_ReleasePointerCapture(This) \
    ((This)->lpVtbl->ReleasePointerCapture(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_SetPointerCapture(This) \
    ((This)->lpVtbl->SetPointerCapture(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_get_HasCapture(This, value) \
    ((This)->lpVtbl->get_HasCapture(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_get_PointerPosition(This, value) \
    ((This)->lpVtbl->get_PointerPosition(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_get_PointerCursor(This, value) \
    ((This)->lpVtbl->get_PointerCursor(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_put_PointerCursor(This, value) \
    ((This)->lpVtbl->put_PointerCursor(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerCaptureLost(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerCaptureLost(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerCaptureLost(This, cookie) \
    ((This)->lpVtbl->remove_PointerCaptureLost(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerEntered(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerEntered(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerEntered(This, cookie) \
    ((This)->lpVtbl->remove_PointerEntered(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerExited(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerExited(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerExited(This, cookie) \
    ((This)->lpVtbl->remove_PointerExited(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerMoved(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerMoved(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerMoved(This, cookie) \
    ((This)->lpVtbl->remove_PointerMoved(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerPressed(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerPressed(This, cookie) \
    ((This)->lpVtbl->remove_PointerPressed(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerReleased(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerReleased(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerReleased(This, cookie) \
    ((This)->lpVtbl->remove_PointerReleased(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_add_PointerWheelChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerWheelChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_remove_PointerWheelChanged(This, cookie) \
    ((This)->lpVtbl->remove_PointerWheelChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICorePointerInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICorePointerInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerInputSource2[] = L"Windows.UI.Core.ICorePointerInputSource2";
typedef struct __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Core.ICorePointerRedirector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICorePointerRedirector[] = L"Windows.UI.Core.ICorePointerRedirector";
typedef struct __x_ABI_CWindows_CUI_CCore_CICorePointerRedirectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PointerRoutedAway)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerRoutedAway)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerRoutedTo)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerRoutedTo)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerRoutedReleased)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CICorePointerRedirector_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerRoutedReleased)(__x_ABI_CWindows_CUI_CCore_CICorePointerRedirector* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICorePointerRedirectorVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICorePointerRedirectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_add_PointerRoutedAway(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerRoutedAway(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_remove_PointerRoutedAway(This, cookie) \
    ((This)->lpVtbl->remove_PointerRoutedAway(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_add_PointerRoutedTo(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerRoutedTo(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_remove_PointerRoutedTo(This, cookie) \
    ((This)->lpVtbl->remove_PointerRoutedTo(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_add_PointerRoutedReleased(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerRoutedReleased(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_remove_PointerRoutedReleased(This, cookie) \
    ((This)->lpVtbl->remove_PointerRoutedReleased(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICorePointerRedirector;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICorePointerRedirector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Core.ICoreTouchHitTesting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreComponentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreTouchHitTesting[] = L"Windows.UI.Core.ICoreTouchHitTesting";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTestingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_TouchHitTesting)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        __FITypedEventHandler_2_IInspectable_Windows__CUI__CCore__CTouchHitTestingEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_TouchHitTesting)(__x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTestingVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTestingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_add_TouchHitTesting(This, handler, pCookie) \
    ((This)->lpVtbl->add_TouchHitTesting(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_remove_TouchHitTesting(This, cookie) \
    ((This)->lpVtbl->remove_TouchHitTesting(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreTouchHitTesting_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow[] = L"Windows.UI.Core.ICoreWindow";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutomationHostProvider)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_Bounds)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_CustomProperties)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Dispatcher)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __x_ABI_CWindows_CUI_CCore_CICoreDispatcher** value);
    HRESULT (STDMETHODCALLTYPE* get_FlowDirection)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreWindowFlowDirection* value);
    HRESULT (STDMETHODCALLTYPE* put_FlowDirection)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreWindowFlowDirection value);
    HRESULT (STDMETHODCALLTYPE* get_IsInputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PointerCursor)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor** value);
    HRESULT (STDMETHODCALLTYPE* put_PointerCursor)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __x_ABI_CWindows_CUI_CCore_CICoreCursor* value);
    HRESULT (STDMETHODCALLTYPE* get_PointerPosition)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Activate)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetAsyncKeyState)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey virtualKey,
        enum __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates* KeyState);
    HRESULT (STDMETHODCALLTYPE* GetKeyState)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey virtualKey,
        enum __x_ABI_CWindows_CUI_CCore_CCoreVirtualKeyStates* KeyState);
    HRESULT (STDMETHODCALLTYPE* ReleasePointerCapture)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* SetPointerCapture)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowActivatedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_AutomationProviderRequested)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CAutomationProviderRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_AutomationProviderRequested)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_CharacterReceived)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCharacterReceivedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_CharacterReceived)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_InputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CInputEnabledEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_InputEnabled)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_KeyDown)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_KeyDown)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_KeyUp)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CKeyEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_KeyUp)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerCaptureLost)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerCaptureLost)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerEntered)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerEntered)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerExited)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerExited)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerMoved)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerMoved)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerPressed)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerPressed)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerReleased)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerReleased)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_TouchHitTesting)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CTouchHitTestingEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_TouchHitTesting)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerWheelChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerWheelChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_SizeChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CWindowSizeChangedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_SizeChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_VisibilityChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CVisibilityChangedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_VisibilityChanged)(__x_ABI_CWindows_CUI_CCore_CICoreWindow* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindow
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_AutomationHostProvider(This, value) \
    ((This)->lpVtbl->get_AutomationHostProvider(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_Bounds(This, value) \
    ((This)->lpVtbl->get_Bounds(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_CustomProperties(This, value) \
    ((This)->lpVtbl->get_CustomProperties(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_Dispatcher(This, value) \
    ((This)->lpVtbl->get_Dispatcher(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_FlowDirection(This, value) \
    ((This)->lpVtbl->get_FlowDirection(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_put_FlowDirection(This, value) \
    ((This)->lpVtbl->put_FlowDirection(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_IsInputEnabled(This, value) \
    ((This)->lpVtbl->get_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_put_IsInputEnabled(This, value) \
    ((This)->lpVtbl->put_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_PointerCursor(This, value) \
    ((This)->lpVtbl->get_PointerCursor(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_put_PointerCursor(This, value) \
    ((This)->lpVtbl->put_PointerCursor(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_PointerPosition(This, value) \
    ((This)->lpVtbl->get_PointerPosition(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_Activate(This) \
    ((This)->lpVtbl->Activate(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_Close(This) \
    ((This)->lpVtbl->Close(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_GetAsyncKeyState(This, virtualKey, KeyState) \
    ((This)->lpVtbl->GetAsyncKeyState(This, virtualKey, KeyState))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_GetKeyState(This, virtualKey, KeyState) \
    ((This)->lpVtbl->GetKeyState(This, virtualKey, KeyState))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_ReleasePointerCapture(This) \
    ((This)->lpVtbl->ReleasePointerCapture(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_SetPointerCapture(This) \
    ((This)->lpVtbl->SetPointerCapture(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_Activated(This, handler, pCookie) \
    ((This)->lpVtbl->add_Activated(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_Activated(This, cookie) \
    ((This)->lpVtbl->remove_Activated(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_AutomationProviderRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_AutomationProviderRequested(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_AutomationProviderRequested(This, cookie) \
    ((This)->lpVtbl->remove_AutomationProviderRequested(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_CharacterReceived(This, handler, pCookie) \
    ((This)->lpVtbl->add_CharacterReceived(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_CharacterReceived(This, cookie) \
    ((This)->lpVtbl->remove_CharacterReceived(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_Closed(This, handler, pCookie) \
    ((This)->lpVtbl->add_Closed(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_Closed(This, cookie) \
    ((This)->lpVtbl->remove_Closed(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_InputEnabled(This, handler, pCookie) \
    ((This)->lpVtbl->add_InputEnabled(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_InputEnabled(This, cookie) \
    ((This)->lpVtbl->remove_InputEnabled(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_KeyDown(This, handler, pCookie) \
    ((This)->lpVtbl->add_KeyDown(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_KeyDown(This, cookie) \
    ((This)->lpVtbl->remove_KeyDown(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_KeyUp(This, handler, pCookie) \
    ((This)->lpVtbl->add_KeyUp(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_KeyUp(This, cookie) \
    ((This)->lpVtbl->remove_KeyUp(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerCaptureLost(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerCaptureLost(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerCaptureLost(This, cookie) \
    ((This)->lpVtbl->remove_PointerCaptureLost(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerEntered(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerEntered(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerEntered(This, cookie) \
    ((This)->lpVtbl->remove_PointerEntered(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerExited(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerExited(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerExited(This, cookie) \
    ((This)->lpVtbl->remove_PointerExited(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerMoved(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerMoved(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerMoved(This, cookie) \
    ((This)->lpVtbl->remove_PointerMoved(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerPressed(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerPressed(This, cookie) \
    ((This)->lpVtbl->remove_PointerPressed(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerReleased(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerReleased(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerReleased(This, cookie) \
    ((This)->lpVtbl->remove_PointerReleased(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_TouchHitTesting(This, handler, pCookie) \
    ((This)->lpVtbl->add_TouchHitTesting(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_TouchHitTesting(This, cookie) \
    ((This)->lpVtbl->remove_TouchHitTesting(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_PointerWheelChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerWheelChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_PointerWheelChanged(This, cookie) \
    ((This)->lpVtbl->remove_PointerWheelChanged(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_SizeChanged(This, handler, pCookie) \
    ((This)->lpVtbl->add_SizeChanged(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_SizeChanged(This, cookie) \
    ((This)->lpVtbl->remove_SizeChanged(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_add_VisibilityChanged(This, handler, pCookie) \
    ((This)->lpVtbl->add_VisibilityChanged(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow_remove_VisibilityChanged(This, cookie) \
    ((This)->lpVtbl->remove_VisibilityChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow2[] = L"Windows.UI.Core.ICoreWindow2";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindow2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PointerPosition)(__x_ABI_CWindows_CUI_CCore_CICoreWindow2* This,
        struct __x_ABI_CWindows_CFoundation_CPoint value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindow2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindow2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindow2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow2_put_PointerPosition(This, value) \
    ((This)->lpVtbl->put_PointerPosition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow3[] = L"Windows.UI.Core.ICoreWindow3";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindow3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ClosestInteractiveBoundsRequested)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CClosestInteractiveBoundsRequestedEventArgs* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ClosestInteractiveBoundsRequested)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* GetCurrentKeyEventDeviceId)(__x_ABI_CWindows_CUI_CCore_CICoreWindow3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindow3Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindow3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindow3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_add_ClosestInteractiveBoundsRequested(This, handler, pCookie) \
    ((This)->lpVtbl->add_ClosestInteractiveBoundsRequested(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_remove_ClosestInteractiveBoundsRequested(This, cookie) \
    ((This)->lpVtbl->remove_ClosestInteractiveBoundsRequested(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow3_GetCurrentKeyEventDeviceId(This, value) \
    ((This)->lpVtbl->GetCurrentKeyEventDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow3;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow4[] = L"Windows.UI.Core.ICoreWindow4";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindow4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ResizeStarted)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ResizeStarted)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_ResizeCompleted)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_IInspectable* handler,
        EventRegistrationToken* pCookie);
    HRESULT (STDMETHODCALLTYPE* remove_ResizeCompleted)(__x_ABI_CWindows_CUI_CCore_CICoreWindow4* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindow4Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindow4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindow4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_add_ResizeStarted(This, handler, pCookie) \
    ((This)->lpVtbl->add_ResizeStarted(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_remove_ResizeStarted(This, cookie) \
    ((This)->lpVtbl->remove_ResizeStarted(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_add_ResizeCompleted(This, handler, pCookie) \
    ((This)->lpVtbl->add_ResizeCompleted(This, handler, pCookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow4_remove_ResizeCompleted(This, cookie) \
    ((This)->lpVtbl->remove_ResizeCompleted(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow4;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Core.ICoreWindow5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindow5[] = L"Windows.UI.Core.ICoreWindow5";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindow5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);
    HRESULT (STDMETHODCALLTYPE* get_ActivationMode)(__x_ABI_CWindows_CUI_CCore_CICoreWindow5* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindow5Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindow5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindow5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindow5_get_ActivationMode(This, value) \
    ((This)->lpVtbl->get_ActivationMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindow5;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindow5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowDialog
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowDialog
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowDialog[] = L"Windows.UI.Core.ICoreWindowDialog";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Showing)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_Showing)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_MaxSize)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_MinSize)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsInteractionDelayed)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInteractionDelayed)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_CancelCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CancelCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BackButtonCommand)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler** value);
    HRESULT (STDMETHODCALLTYPE* put_BackButtonCommand)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* value);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialog* This,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_add_Showing(This, handler, cookie) \
    ((This)->lpVtbl->add_Showing(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_remove_Showing(This, cookie) \
    ((This)->lpVtbl->remove_Showing(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_MaxSize(This, value) \
    ((This)->lpVtbl->get_MaxSize(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_MinSize(This, value) \
    ((This)->lpVtbl->get_MinSize(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_IsInteractionDelayed(This, value) \
    ((This)->lpVtbl->get_IsInteractionDelayed(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_put_IsInteractionDelayed(This, value) \
    ((This)->lpVtbl->put_IsInteractionDelayed(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->get_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_put_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->put_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_CancelCommandIndex(This, value) \
    ((This)->lpVtbl->get_CancelCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_put_CancelCommandIndex(This, value) \
    ((This)->lpVtbl->put_CancelCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_get_BackButtonCommand(This, value) \
    ((This)->lpVtbl->get_BackButtonCommand(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_put_BackButtonCommand(This, value) \
    ((This)->lpVtbl->put_BackButtonCommand(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_ShowAsync(This, asyncInfo) \
    ((This)->lpVtbl->ShowAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowDialog;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialog_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowDialogFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowDialog
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowDialogFactory[] = L"Windows.UI.Core.ICoreWindowDialogFactory";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithTitle)(__x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory* This,
        HSTRING title,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowDialog** coreWindowDialog);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactoryVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_CreateWithTitle(This, title, coreWindowDialog) \
    ((This)->lpVtbl->CreateWithTitle(This, title, coreWindowDialog))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowDialogFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowEventArgs[] = L"Windows.UI.Core.ICoreWindowEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowFlyout
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowFlyout
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFlyout[] = L"Windows.UI.Core.ICoreWindowFlyout";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Showing)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        __FITypedEventHandler_2_Windows__CUI__CCore__CCoreWindow_Windows__CUI__CCore__CCoreWindowPopupShowingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_Showing)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_MaxSize)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_MinSize)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsInteractionDelayed)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInteractionDelayed)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        __FIVector_1_Windows__CUI__CPopups__CIUICommand** value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DefaultCommandIndex)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BackButtonCommand)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler** value);
    HRESULT (STDMETHODCALLTYPE* put_BackButtonCommand)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* value);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout* This,
        __FIAsyncOperation_1_Windows__CUI__CPopups__CIUICommand** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_add_Showing(This, handler, cookie) \
    ((This)->lpVtbl->add_Showing(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_remove_Showing(This, cookie) \
    ((This)->lpVtbl->remove_Showing(This, cookie))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_MaxSize(This, value) \
    ((This)->lpVtbl->get_MaxSize(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_MinSize(This, value) \
    ((This)->lpVtbl->get_MinSize(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_IsInteractionDelayed(This, value) \
    ((This)->lpVtbl->get_IsInteractionDelayed(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_put_IsInteractionDelayed(This, value) \
    ((This)->lpVtbl->put_IsInteractionDelayed(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->get_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_put_DefaultCommandIndex(This, value) \
    ((This)->lpVtbl->put_DefaultCommandIndex(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_get_BackButtonCommand(This, value) \
    ((This)->lpVtbl->get_BackButtonCommand(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_put_BackButtonCommand(This, value) \
    ((This)->lpVtbl->put_BackButtonCommand(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_ShowAsync(This, asyncInfo) \
    ((This)->lpVtbl->ShowAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowFlyoutFactory
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowFlyout
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowFlyoutFactory[] = L"Windows.UI.Core.ICoreWindowFlyoutFactory";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        struct __x_ABI_CWindows_CFoundation_CPoint position,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout** coreWindowFlyout);
    HRESULT (STDMETHODCALLTYPE* CreateWithTitle)(__x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory* This,
        struct __x_ABI_CWindows_CFoundation_CPoint position,
        HSTRING title,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyout** coreWindowFlyout);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactoryVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_Create(This, position, coreWindowFlyout) \
    ((This)->lpVtbl->Create(This, position, coreWindowFlyout))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_CreateWithTitle(This, position, title, coreWindowFlyout) \
    ((This)->lpVtbl->CreateWithTitle(This, position, title, coreWindowFlyout))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowFlyoutFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowPopupShowingEventArgs
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowPopupShowingEventArgs
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowPopupShowingEventArgs[] = L"Windows.UI.Core.ICoreWindowPopupShowingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetDesiredSize)(__x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_SetDesiredSize(This, value) \
    ((This)->lpVtbl->SetDesiredSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowPopupShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManager[] = L"Windows.UI.Core.ICoreWindowResizeManager";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* NotifyLayoutCompleted)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_NotifyLayoutCompleted(This) \
    ((This)->lpVtbl->NotifyLayoutCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManagerLayoutCapability[] = L"Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ShouldWaitForLayoutCompletion)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShouldWaitForLayoutCompletion)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapabilityVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_put_ShouldWaitForLayoutCompletion(This, value) \
    ((This)->lpVtbl->put_ShouldWaitForLayoutCompletion(This, value))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_get_ShouldWaitForLayoutCompletion(This, value) \
    ((This)->lpVtbl->get_ShouldWaitForLayoutCompletion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerLayoutCapability_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowResizeManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindowResizeManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowResizeManagerStatics[] = L"Windows.UI.Core.ICoreWindowResizeManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManager** CoreWindowResizeManager);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_GetForCurrentView(This, CoreWindowResizeManager) \
    ((This)->lpVtbl->GetForCurrentView(This, CoreWindowResizeManager))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowResizeManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowStatic[] = L"Windows.UI.Core.ICoreWindowStatic";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentThread)(__x_ABI_CWindows_CUI_CCore_CICoreWindowStatic* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow** ppWindow);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowStaticVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_GetForCurrentThread(This, ppWindow) \
    ((This)->lpVtbl->GetForCurrentThread(This, ppWindow))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowStatic;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ICoreWindowWithContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.CoreWindow
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ICoreWindowWithContext[] = L"Windows.UI.Core.ICoreWindowWithContext";
typedef struct __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UIContext)(__x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext* This,
        __x_ABI_CWindows_CUI_CIUIContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContextVtbl;

interface __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_get_UIContext(This, value) \
    ((This)->lpVtbl->get_UIContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CICoreWindowWithContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Core.IIdleDispatchedHandlerArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.IdleDispatchedHandlerArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IIdleDispatchedHandlerArgs[] = L"Windows.UI.Core.IIdleDispatchedHandlerArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDispatcherIdle)(__x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_get_IsDispatcherIdle(This, value) \
    ((This)->lpVtbl->get_IsDispatcherIdle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIIdleDispatchedHandlerArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IInitializeWithCoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IInitializeWithCoreWindow[] = L"Windows.UI.Core.IInitializeWithCoreWindow";
typedef struct __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindowVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Initialize)(__x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* window);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindowVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindowVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_Initialize(This, window) \
    ((This)->lpVtbl->Initialize(This, window))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIInitializeWithCoreWindow_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IInputEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.InputEnabledEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IInputEnabledEventArgs[] = L"Windows.UI.Core.IInputEnabledEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputEnabled)(__x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_get_InputEnabled(This, value) \
    ((This)->lpVtbl->get_InputEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIInputEnabledEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.KeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IKeyEventArgs[] = L"Windows.UI.Core.IKeyEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIKeyEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VirtualKey)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKey* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyStatus)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCorePhysicalKeyStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIKeyEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIKeyEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_get_VirtualKey(This, value) \
    ((This)->lpVtbl->get_VirtualKey(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_get_KeyStatus(This, value) \
    ((This)->lpVtbl->get_KeyStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIKeyEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IKeyEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.KeyEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IKeyEventArgs2[] = L"Windows.UI.Core.IKeyEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIKeyEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Core.IPointerEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.PointerEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IPointerEventArgs[] = L"Windows.UI.Core.IPointerEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPoint)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyModifiers)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CVirtualKeyModifiers* value);
    HRESULT (STDMETHODCALLTYPE* GetIntermediatePoints)(__x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* This,
        __FIVector_1_Windows__CUI__CInput__CPointerPoint** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIPointerEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_get_CurrentPoint(This, value) \
    ((This)->lpVtbl->get_CurrentPoint(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_get_KeyModifiers(This, value) \
    ((This)->lpVtbl->get_KeyModifiers(This, value))

#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_GetIntermediatePoints(This, value) \
    ((This)->lpVtbl->GetIntermediatePoints(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManager[] = L"Windows.UI.Core.ISystemNavigationManager";
typedef struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_BackRequested)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        __FIEventHandler_1_Windows__CUI__CCore__CBackRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BackRequested)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerVtbl;

interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_add_BackRequested(This, handler, token) \
    ((This)->lpVtbl->add_BackRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_remove_BackRequested(This, token) \
    ((This)->lpVtbl->remove_BackRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManager2[] = L"Windows.UI.Core.ISystemNavigationManager2";
typedef struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppViewBackButtonVisibility)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        enum __x_ABI_CWindows_CUI_CCore_CAppViewBackButtonVisibility* value);
    HRESULT (STDMETHODCALLTYPE* put_AppViewBackButtonVisibility)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2* This,
        enum __x_ABI_CWindows_CUI_CCore_CAppViewBackButtonVisibility value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2Vtbl;

interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_get_AppViewBackButtonVisibility(This, value) \
    ((This)->lpVtbl->get_AppViewBackButtonVisibility(This, value))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_put_AppViewBackButtonVisibility(This, value) \
    ((This)->lpVtbl->put_AppViewBackButtonVisibility(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ISystemNavigationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.SystemNavigationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ISystemNavigationManagerStatics[] = L"Windows.UI.Core.ISystemNavigationManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics* This,
        __x_ABI_CWindows_CUI_CCore_CISystemNavigationManager** loader);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_GetForCurrentView(This, loader) \
    ((This)->lpVtbl->GetForCurrentView(This, loader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CISystemNavigationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.ITouchHitTestingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.TouchHitTestingEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_ITouchHitTestingEventArgs[] = L"Windows.UI.Core.ITouchHitTestingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProximityEvaluation)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation* value);
    HRESULT (STDMETHODCALLTYPE* put_ProximityEvaluation)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation value);
    HRESULT (STDMETHODCALLTYPE* get_Point)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingBox)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* EvaluateProximityToRect)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect controlBoundingBox,
        struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation* proximityEvaluation);
    HRESULT (STDMETHODCALLTYPE* EvaluateProximityToPolygon)(__x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs* This,
        UINT32 controlVerticesLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* controlVertices,
        struct __x_ABI_CWindows_CUI_CCore_CCoreProximityEvaluation* proximityEvaluation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_get_ProximityEvaluation(This, value) \
    ((This)->lpVtbl->get_ProximityEvaluation(This, value))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_put_ProximityEvaluation(This, value) \
    ((This)->lpVtbl->put_ProximityEvaluation(This, value))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_get_Point(This, value) \
    ((This)->lpVtbl->get_Point(This, value))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_get_BoundingBox(This, value) \
    ((This)->lpVtbl->get_BoundingBox(This, value))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_EvaluateProximityToRect(This, controlBoundingBox, proximityEvaluation) \
    ((This)->lpVtbl->EvaluateProximityToRect(This, controlBoundingBox, proximityEvaluation))

#define __x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_EvaluateProximityToPolygon(This, controlVerticesLength, controlVertices, proximityEvaluation) \
    ((This)->lpVtbl->EvaluateProximityToPolygon(This, controlVerticesLength, controlVertices, proximityEvaluation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CITouchHitTestingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IVisibilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.VisibilityChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IVisibilityChangedEventArgs[] = L"Windows.UI.Core.IVisibilityChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIVisibilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IWindowActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.WindowActivatedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IWindowActivatedEventArgs[] = L"Windows.UI.Core.IWindowActivatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowActivationState)(__x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreWindowActivationState* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_get_WindowActivationState(This, value) \
    ((This)->lpVtbl->get_WindowActivationState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIWindowActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Core.IWindowSizeChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Core.WindowSizeChangedEventArgs
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Core.ICoreWindowEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Core_IWindowSizeChangedEventArgs[] = L"Windows.UI.Core.IWindowSizeChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CCore_CIWindowSizeChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.AcceleratorKeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IAcceleratorKeyEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *    Windows.UI.Core.IAcceleratorKeyEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_AcceleratorKeyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_AcceleratorKeyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_AcceleratorKeyEventArgs[] = L"Windows.UI.Core.AcceleratorKeyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.AutomationProviderRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IAutomationProviderRequestedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_AutomationProviderRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_AutomationProviderRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_AutomationProviderRequestedEventArgs[] = L"Windows.UI.Core.AutomationProviderRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.BackRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IBackRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_BackRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_BackRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_BackRequestedEventArgs[] = L"Windows.UI.Core.BackRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CharacterReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICharacterReceivedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CharacterReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CharacterReceivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CharacterReceivedEventArgs[] = L"Windows.UI.Core.CharacterReceivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_ClosestInteractiveBoundsRequestedEventArgs[] = L"Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Core.CoreAcceleratorKeys
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreAcceleratorKeys ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreAcceleratorKeys_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreAcceleratorKeys_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreAcceleratorKeys[] = L"Windows.UI.Core.CoreAcceleratorKeys";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreComponentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreInputSourceBase ** Default Interface **
 *    Windows.UI.Core.ICorePointerInputSource
 *    Windows.UI.Core.ICoreKeyboardInputSource
 *    Windows.UI.Core.ICoreComponentFocusable
 *    Windows.UI.Core.ICoreTouchHitTesting
 *    Windows.UI.Core.ICoreClosestInteractiveBoundsRequested
 *    Windows.UI.Core.ICoreKeyboardInputSource2
 *    Windows.UI.Core.ICorePointerInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreComponentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreComponentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreComponentInputSource[] = L"Windows.UI.Core.CoreComponentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreCursor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Core.ICoreCursorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreCursor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreCursor_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreCursor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreCursor[] = L"Windows.UI.Core.CoreCursor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreDispatcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreDispatcher ** Default Interface **
 *    Windows.UI.Core.ICoreAcceleratorKeys
 *    Windows.UI.Core.ICoreDispatcherWithTaskPriority
 *    Windows.UI.Core.ICoreDispatcher2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreDispatcher_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreDispatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreDispatcher[] = L"Windows.UI.Core.CoreDispatcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreInputSourceBase ** Default Interface **
 *    Windows.UI.Core.ICorePointerInputSource
 *    Windows.UI.Core.ICorePointerInputSource2
 *    Windows.UI.Core.ICorePointerRedirector
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreIndependentInputSource[] = L"Windows.UI.Core.CoreIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreIndependentInputSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreIndependentInputSourceControllerStatics interface starting with version 13.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreIndependentInputSourceController ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSourceController_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreIndependentInputSourceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreIndependentInputSourceController[] = L"Windows.UI.Core.CoreIndependentInputSourceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.UI.Core.CoreWindow
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreWindowStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindow ** Default Interface **
 *    Windows.UI.Core.ICoreWindow2
 *    Windows.UI.Core.ICorePointerRedirector
 *    Windows.UI.Core.ICoreWindow3
 *    Windows.UI.Core.ICoreWindow4
 *    Windows.UI.Core.ICoreWindow5
 *    Windows.UI.Core.ICoreWindowWithContext
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindow_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindow_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindow[] = L"Windows.UI.Core.CoreWindow";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowDialog
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *   Type can be activated via the Windows.UI.Core.ICoreWindowDialogFactory interface starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowDialog ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowDialog_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowDialog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowDialog[] = L"Windows.UI.Core.CoreWindowDialog";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowEventArgs[] = L"Windows.UI.Core.CoreWindowEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowFlyout
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Core.ICoreWindowFlyoutFactory interface starting with version 1.0 of the Windows.UI.Core.CoreWindowDialogsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowFlyout ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowFlyout_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowFlyout_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowFlyout[] = L"Windows.UI.Core.CoreWindowFlyout";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowPopupShowingEventArgs
 *
 * Introduced to Windows.UI.Core.CoreWindowDialogsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowPopupShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowPopupShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowPopupShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowPopupShowingEventArgs[] = L"Windows.UI.Core.CoreWindowPopupShowingEventArgs";
#endif
#endif // WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.CoreWindowResizeManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ICoreWindowResizeManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ICoreWindowResizeManager ** Default Interface **
 *    Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_CoreWindowResizeManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_CoreWindowResizeManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_CoreWindowResizeManager[] = L"Windows.UI.Core.CoreWindowResizeManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.IdleDispatchedHandlerArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IIdleDispatchedHandlerArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_IdleDispatchedHandlerArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_IdleDispatchedHandlerArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_IdleDispatchedHandlerArgs[] = L"Windows.UI.Core.IdleDispatchedHandlerArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.InputEnabledEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IInputEnabledEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_InputEnabledEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_InputEnabledEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_InputEnabledEventArgs[] = L"Windows.UI.Core.InputEnabledEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.KeyEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IKeyEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *    Windows.UI.Core.IKeyEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_KeyEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_KeyEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_KeyEventArgs[] = L"Windows.UI.Core.KeyEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.PointerEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IPointerEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_PointerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_PointerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_PointerEventArgs[] = L"Windows.UI.Core.PointerEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.SystemNavigationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Core.ISystemNavigationManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ISystemNavigationManager ** Default Interface **
 *    Windows.UI.Core.ISystemNavigationManager2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_SystemNavigationManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_SystemNavigationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_SystemNavigationManager[] = L"Windows.UI.Core.SystemNavigationManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.TouchHitTestingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.ITouchHitTestingEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_TouchHitTestingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_TouchHitTestingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_TouchHitTestingEventArgs[] = L"Windows.UI.Core.TouchHitTestingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.VisibilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IVisibilityChangedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_VisibilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_VisibilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_VisibilityChangedEventArgs[] = L"Windows.UI.Core.VisibilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.WindowActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IWindowActivatedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_WindowActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_WindowActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_WindowActivatedEventArgs[] = L"Windows.UI.Core.WindowActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Core.WindowSizeChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Core.IWindowSizeChangedEventArgs ** Default Interface **
 *    Windows.UI.Core.ICoreWindowEventArgs
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Core_WindowSizeChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Core_WindowSizeChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Core_WindowSizeChangedEventArgs[] = L"Windows.UI.Core.WindowSizeChangedEventArgs";
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
#endif // __windows2Eui2Ecore_p_h__

#endif // __windows2Eui2Ecore_h__
