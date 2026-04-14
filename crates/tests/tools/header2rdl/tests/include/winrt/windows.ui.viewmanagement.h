
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
#ifndef __windows2Eui2Eviewmanagement_h__
#define __windows2Eui2Eviewmanagement_h__
#ifndef __windows2Eui2Eviewmanagement_p_h__
#define __windows2Eui2Eviewmanagement_p_h__


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

#if !defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)
#define WINDOWS_PHONE_PHONECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONECONTRACT_VERSION)

#if !defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)
#define WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_PHONEINTERNALCONTRACT_VERSION)

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#if !defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)
#define WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.Phone.h"
#include "Windows.UI.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Popups.h"
#include "Windows.UI.WindowManagement.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IAccessibilitySettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings ABI::Windows::UI::ViewManagement::IAccessibilitySettings

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IActivationViewSwitcher;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher ABI::Windows::UI::ViewManagement::IActivationViewSwitcher

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView ABI::Windows::UI::ViewManagement::IApplicationView

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2 ABI::Windows::UI::ViewManagement::IApplicationView2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView3;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3 ABI::Windows::UI::ViewManagement::IApplicationView3

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView4;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4 ABI::Windows::UI::ViewManagement::IApplicationView4

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView7;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7 ABI::Windows::UI::ViewManagement::IApplicationView7

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationView9;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9 ABI::Windows::UI::ViewManagement::IApplicationView9

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewConsolidatedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs ABI::Windows::UI::ViewManagement::IApplicationViewConsolidatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewConsolidatedEventArgs2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2 ABI::Windows::UI::ViewManagement::IApplicationViewConsolidatedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewFullscreenStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics ABI::Windows::UI::ViewManagement::IApplicationViewFullscreenStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewInteropStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics ABI::Windows::UI::ViewManagement::IApplicationViewInteropStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewScaling;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling ABI::Windows::UI::ViewManagement::IApplicationViewScaling

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewScalingStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics ABI::Windows::UI::ViewManagement::IApplicationViewScalingStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics ABI::Windows::UI::ViewManagement::IApplicationViewStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewStatics2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2 ABI::Windows::UI::ViewManagement::IApplicationViewStatics2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewStatics3;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3 ABI::Windows::UI::ViewManagement::IApplicationViewStatics3

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewStatics4;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4 ABI::Windows::UI::ViewManagement::IApplicationViewStatics4

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewSwitcherStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics ABI::Windows::UI::ViewManagement::IApplicationViewSwitcherStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewSwitcherStatics2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2 ABI::Windows::UI::ViewManagement::IApplicationViewSwitcherStatics2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewSwitcherStatics3;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3 ABI::Windows::UI::ViewManagement::IApplicationViewSwitcherStatics3

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewTitleBar;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar ABI::Windows::UI::ViewManagement::IApplicationViewTitleBar

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewTransferContext;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext ABI::Windows::UI::ViewManagement::IApplicationViewTransferContext

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewTransferContextStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics ABI::Windows::UI::ViewManagement::IApplicationViewTransferContextStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IApplicationViewWithContext;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext ABI::Windows::UI::ViewManagement::IApplicationViewWithContext

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPane;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane ABI::Windows::UI::ViewManagement::IInputPane

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPane2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2 ABI::Windows::UI::ViewManagement::IInputPane2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPaneControl;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl ABI::Windows::UI::ViewManagement::IInputPaneControl

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPaneStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics ABI::Windows::UI::ViewManagement::IInputPaneStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPaneStatics2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2 ABI::Windows::UI::ViewManagement::IInputPaneStatics2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IInputPaneVisibilityEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs ABI::Windows::UI::ViewManagement::IInputPaneVisibilityEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IProjectionManagerStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics ABI::Windows::UI::ViewManagement::IProjectionManagerStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IProjectionManagerStatics2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2 ABI::Windows::UI::ViewManagement::IProjectionManagerStatics2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IStatusBar;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar ABI::Windows::UI::ViewManagement::IStatusBar

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IStatusBarProgressIndicator;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator ABI::Windows::UI::ViewManagement::IStatusBarProgressIndicator

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IStatusBarStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics ABI::Windows::UI::ViewManagement::IStatusBarStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings ABI::Windows::UI::ViewManagement::IUISettings

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings2;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2 ABI::Windows::UI::ViewManagement::IUISettings2

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings3;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3 ABI::Windows::UI::ViewManagement::IUISettings3

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings4;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4 ABI::Windows::UI::ViewManagement::IUISettings4

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings5;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5 ABI::Windows::UI::ViewManagement::IUISettings5

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettings6;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6 ABI::Windows::UI::ViewManagement::IUISettings6

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettingsAnimationsEnabledChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs ABI::Windows::UI::ViewManagement::IUISettingsAnimationsEnabledChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettingsAutoHideScrollBarsChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs ABI::Windows::UI::ViewManagement::IUISettingsAutoHideScrollBarsChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUISettingsMessageDurationChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs ABI::Windows::UI::ViewManagement::IUISettingsMessageDurationChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUIViewSettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings ABI::Windows::UI::ViewManagement::IUIViewSettings

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUIViewSettingsPreferredInteractionMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode ABI::Windows::UI::ViewManagement::IUIViewSettingsPreferredInteractionMode

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IUIViewSettingsStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics ABI::Windows::UI::ViewManagement::IUIViewSettingsStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IViewModePreferences;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences ABI::Windows::UI::ViewManagement::IViewModePreferences

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                interface IViewModePreferencesStatics;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics ABI::Windows::UI::ViewManagement::IViewModePreferencesStatics

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__

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
                class DisplayRegion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

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



#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */


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
            namespace ViewManagement {
                class AccessibilitySettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f5917e6f-5abf-5e65-b5b4-1b9c8d94e788"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::AccessibilitySettings*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::AccessibilitySettings*, ABI::Windows::UI::ViewManagement::IAccessibilitySettings*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.AccessibilitySettings, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::AccessibilitySettings*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class ApplicationView;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("00c1f983-c836-565c-8bbf-7053055bdb4c"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::ApplicationView*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::ApplicationView*, ABI::Windows::UI::ViewManagement::IApplicationView*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.ApplicationView, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::ApplicationView*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class ApplicationViewConsolidatedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("463c606a-8c82-5a29-a2bd-040781f25348"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::ApplicationView*, ABI::Windows::UI::ViewManagement::ApplicationViewConsolidatedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::ApplicationView*, ABI::Windows::UI::ViewManagement::IApplicationView*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::ApplicationViewConsolidatedEventArgs*, ABI::Windows::UI::ViewManagement::IApplicationViewConsolidatedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.ApplicationView, Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::ApplicationView*, ABI::Windows::UI::ViewManagement::ApplicationViewConsolidatedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class InputPane;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class InputPaneVisibilityEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b813d684-d953-5a8a-9b30-78b79fb9147b"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::InputPane*, ABI::Windows::UI::ViewManagement::InputPaneVisibilityEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::InputPane*, ABI::Windows::UI::ViewManagement::IInputPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::InputPaneVisibilityEventArgs*, ABI::Windows::UI::ViewManagement::IInputPaneVisibilityEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.InputPane, Windows.UI.ViewManagement.InputPaneVisibilityEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::InputPane*, ABI::Windows::UI::ViewManagement::InputPaneVisibilityEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class StatusBar;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("38254d6e-8596-5218-bb61-5d9db935aa02"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::StatusBar*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::StatusBar*, ABI::Windows::UI::ViewManagement::IStatusBar*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.StatusBar, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::StatusBar*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_USE */

#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class UISettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2dbdba9d-20da-519d-9078-09f835bc5bc7"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::IUISettings*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.UISettings, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class UISettingsAnimationsEnabledChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("deff0f90-59e1-5859-a245-3d275081c2ad"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsAnimationsEnabledChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::IUISettings*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettingsAnimationsEnabledChangedEventArgs*, ABI::Windows::UI::ViewManagement::IUISettingsAnimationsEnabledChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.UISettings, Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsAnimationsEnabledChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class UISettingsAutoHideScrollBarsChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("808aef30-2660-51b0-9c11-f75dd42006b4"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsAutoHideScrollBarsChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::IUISettings*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettingsAutoHideScrollBarsChangedEventArgs*, ABI::Windows::UI::ViewManagement::IUISettingsAutoHideScrollBarsChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.UISettings, Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsAutoHideScrollBarsChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class UISettingsMessageDurationChangedEventArgs;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec807b08-1ac6-5b37-9af7-1aaf1c93577e"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsMessageDurationChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::IUISettings*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UISettingsMessageDurationChangedEventArgs*, ABI::Windows::UI::ViewManagement::IUISettingsMessageDurationChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.UISettings, Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::UISettings*, ABI::Windows::UI::ViewManagement::UISettingsMessageDurationChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class UIViewSettings;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bf3e2a14-6fce-5bb3-838e-de9a00e11aad"))
ITypedEventHandler<ABI::Windows::UI::ViewManagement::UIViewSettings*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ViewManagement::UIViewSettings*, ABI::Windows::UI::ViewManagement::IUIViewSettings*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ViewManagement.UIViewSettings, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ViewManagement::UIViewSettings*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                class DeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                interface IDeviceInformation;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation ABI::Windows::Devices::Enumeration::IDeviceInformation

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

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
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

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
                class WindowingEnvironment;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewBoundsMode : int ApplicationViewBoundsMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewMode : int ApplicationViewMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewOrientation : int ApplicationViewOrientation;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewState : int ApplicationViewState;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewSwitchingOptions : unsigned int ApplicationViewSwitchingOptions;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ApplicationViewWindowingMode : int ApplicationViewWindowingMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum FullScreenSystemOverlayMode : int FullScreenSystemOverlayMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum HandPreference : int HandPreference;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum UIColorType : int UIColorType;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum UIElementType : int UIElementType;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum UserInteractionMode : int UserInteractionMode;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum ViewSizePreference : int ViewSizePreference;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class ApplicationViewTitleBar;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class StatusBarProgressIndicator;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                class ViewModePreferences;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewBoundsMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ApplicationViewBoundsMode : int
                {
                    ApplicationViewBoundsMode_UseVisible = 0,
                    ApplicationViewBoundsMode_UseCoreWindow = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ApplicationViewMode : int
                {
                    ApplicationViewMode_Default = 0,
                    ApplicationViewMode_CompactOverlay = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ApplicationViewOrientation : int
                {
                    ApplicationViewOrientation_Landscape = 0,
                    ApplicationViewOrientation_Portrait = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("ApplicationViewState may be altered or unavailable for releases after Windows 8.1. Instead, query for window layout sizes directly.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                ApplicationViewState : int
                {
                    ApplicationViewState_FullScreenLandscape = 0,
                    ApplicationViewState_Filled = 1,
                    ApplicationViewState_Snapped = 2,
                    ApplicationViewState_FullScreenPortrait = 3,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewSwitchingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ApplicationViewSwitchingOptions : unsigned int
                {
                    ApplicationViewSwitchingOptions_Default = 0,
                    ApplicationViewSwitchingOptions_SkipAnimation = 0x1,
                    ApplicationViewSwitchingOptions_ConsolidateViews = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(ApplicationViewSwitchingOptions)
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewWindowingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ApplicationViewWindowingMode : int
                {
                    ApplicationViewWindowingMode_Auto = 0,
                    ApplicationViewWindowingMode_PreferredLaunchViewSize = 1,
                    ApplicationViewWindowingMode_FullScreen = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    ApplicationViewWindowingMode_CompactOverlay = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    ApplicationViewWindowingMode_Maximized = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.FullScreenSystemOverlayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum FullScreenSystemOverlayMode : int
                {
                    FullScreenSystemOverlayMode_Standard = 0,
                    FullScreenSystemOverlayMode_Minimal = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.HandPreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum HandPreference : int
                {
                    HandPreference_LeftHanded = 0,
                    HandPreference_RightHanded = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ScreenCaptureDisabledBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ScreenCaptureDisabledBehavior : int
                {
                    ScreenCaptureDisabledBehavior_DrawAsBlack = 0,
                    ScreenCaptureDisabledBehavior_ExcludeFromCapture = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.UI.ViewManagement.UIColorType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum UIColorType : int
                {
                    UIColorType_Background = 0,
                    UIColorType_Foreground = 1,
                    UIColorType_AccentDark3 = 2,
                    UIColorType_AccentDark2 = 3,
                    UIColorType_AccentDark1 = 4,
                    UIColorType_Accent = 5,
                    UIColorType_AccentLight1 = 6,
                    UIColorType_AccentLight2 = 7,
                    UIColorType_AccentLight3 = 8,
                    UIColorType_Complement = 9,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.UIElementType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum UIElementType : int
                {
                    UIElementType_ActiveCaption = 0,
                    UIElementType_Background = 1,
                    UIElementType_ButtonFace = 2,
                    UIElementType_ButtonText = 3,
                    UIElementType_CaptionText = 4,
                    UIElementType_GrayText = 5,
                    UIElementType_Highlight = 6,
                    UIElementType_HighlightText = 7,
                    UIElementType_Hotlight = 8,
                    UIElementType_InactiveCaption = 9,
                    UIElementType_InactiveCaptionText = 10,
                    UIElementType_Window = 11,
                    UIElementType_WindowText = 12,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_AccentColor = 1000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_TextHigh = 1001,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_TextMedium = 1002,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_TextLow = 1003,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_TextContrastWithHigh = 1004,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_NonTextHigh = 1005,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_NonTextMediumHigh = 1006,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_NonTextMedium = 1007,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_NonTextMediumLow = 1008,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_NonTextLow = 1009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_PageBackground = 1010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_PopupBackground = 1011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    UIElementType_OverlayOutsidePopup = 1012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.UserInteractionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum UserInteractionMode : int
                {
                    UserInteractionMode_Mouse = 0,
                    UserInteractionMode_Touch = 1,
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ViewSizePreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                enum ViewSizePreference : int
                {
                    ViewSizePreference_Default = 0,
                    ViewSizePreference_UseLess = 1,
                    ViewSizePreference_UseHalf = 2,
                    ViewSizePreference_UseMore = 3,
                    ViewSizePreference_UseMinimum = 4,
                    ViewSizePreference_UseNone = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    ViewSizePreference_Custom = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                };
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IAccessibilitySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.AccessibilitySettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IAccessibilitySettings[] = L"Windows.UI.ViewManagement.IAccessibilitySettings";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("fe0e8147-c4c0-4562-b962-1327b52ad5b9")
                IAccessibilitySettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HighContrast(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HighContrastScheme(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HighContrastChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HighContrastChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccessibilitySettings = __uuidof(IAccessibilitySettings);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IActivationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ActivationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IActivationViewSwitcher[] = L"Windows.UI.ViewManagement.IActivationViewSwitcher";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("dca71bb6-7350-492b-aac7-c8a13d7224ad")
                IActivationViewSwitcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAsStandaloneAsync(
                        INT32 viewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAsStandaloneWithSizePreferenceAsync(
                        INT32 viewId,
                        ABI::Windows::UI::ViewManagement::ViewSizePreference sizePreference,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsViewPresentedOnActivationVirtualDesktop(
                        INT32 viewId,
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationViewSwitcher = __uuidof(IActivationViewSwitcher);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView[] = L"Windows.UI.ViewManagement.IApplicationView";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("d222d519-4361-451e-96c4-60f4f9742db0")
                IApplicationView : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::UI::ViewManagement::ApplicationViewOrientation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdjacentToLeftDisplayEdge(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdjacentToRightDisplayEdge(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("To check full screen mode, use IsFullScreenMode. To check if the view is adjacent to both edges, use AdjacentToLeftDisplayEdge and AdjacentToRightDisplayEdge. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_IsFullScreen(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOnLockScreen(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsScreenCaptureEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsScreenCaptureEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Consolidated(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Consolidated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView = __uuidof(IApplicationView);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView2[] = L"Windows.UI.ViewManagement.IApplicationView2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("e876b196-a545-40dc-b594-450cba68cc00")
                IApplicationView2 : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_SuppressSystemOverlays(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_SuppressSystemOverlays(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VisibleBounds(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_VisibleBoundsChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_VisibleBoundsChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDesiredBoundsMode(
                        ABI::Windows::UI::ViewManagement::ApplicationViewBoundsMode boundsMode,
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredBoundsMode(
                        ABI::Windows::UI::ViewManagement::ApplicationViewBoundsMode* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView2 = __uuidof(IApplicationView2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView3[] = L"Windows.UI.ViewManagement.IApplicationView3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("903c9ce5-793a-4fdf-a2b2-af1ac21e3108")
                IApplicationView3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TitleBar(
                        ABI::Windows::UI::ViewManagement::IApplicationViewTitleBar** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FullScreenSystemOverlayMode(
                        ABI::Windows::UI::ViewManagement::FullScreenSystemOverlayMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FullScreenSystemOverlayMode(
                        ABI::Windows::UI::ViewManagement::FullScreenSystemOverlayMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsFullScreenMode(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryEnterFullScreenMode(
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ExitFullScreenMode(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowStandardSystemOverlays(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryResizeView(
                        ABI::Windows::Foundation::Size value,
                        boolean* success
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPreferredMinSize(
                        ABI::Windows::Foundation::Size minSize
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView3 = __uuidof(IApplicationView3);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView4[] = L"Windows.UI.ViewManagement.IApplicationView4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("15e5cbec-9e0f-46b5-bc3f-9bf653e74b5e")
                IApplicationView4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewMode(
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsViewModeSupported(
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode viewMode,
                        boolean* isViewModeSupported
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryEnterViewModeAsync(
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode viewMode,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryEnterViewModeWithPreferencesAsync(
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode viewMode,
                        ABI::Windows::UI::ViewManagement::IViewModePreferences* viewModePreferences,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryConsolidateAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView4 = __uuidof(IApplicationView4);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView7[] = L"Windows.UI.ViewManagement.IApplicationView7";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("a0369647-5faf-5aa6-9c38-befbb12a071e")
                IApplicationView7 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PersistedStateId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PersistedStateId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView7 = __uuidof(IApplicationView7);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView9[] = L"Windows.UI.ViewManagement.IApplicationView9";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("9c6516f9-021a-5f01-93e5-9bdad2647574")
                IApplicationView9 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowingEnvironment(
                        ABI::Windows::UI::WindowManagement::IWindowingEnvironment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDisplayRegions(
                        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationView9 = __uuidof(IApplicationView9);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewConsolidatedEventArgs[] = L"Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("514449ec-7ea2-4de7-a6a6-7dfbaaebb6fb")
                IApplicationViewConsolidatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsUserInitiated(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewConsolidatedEventArgs = __uuidof(IApplicationViewConsolidatedEventArgs);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewConsolidatedEventArgs2[] = L"Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("1c199ecc-6dc1-40f4-afee-07d9ea296430")
                IApplicationViewConsolidatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAppInitiated(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewConsolidatedEventArgs2 = __uuidof(IApplicationViewConsolidatedEventArgs2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewFullscreenStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewFullscreenStatics[] = L"Windows.UI.ViewManagement.IApplicationViewFullscreenStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("bc792ebd-64fe-4b65-a0c0-901ce2b68636")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("IApplicationViewFullscreenStatics is deprecated after Windows 8. Please use other resize APIs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IApplicationViewFullscreenStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("IApplicationViewFullscreenStatics is deprecated after Windows 8. Please use other resize APIs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE TryUnsnapToFullscreen(
                        boolean* success
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewFullscreenStatics = __uuidof(IApplicationViewFullscreenStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewInteropStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewInteropStatics[] = L"Windows.UI.ViewManagement.IApplicationViewInteropStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("c446fb5d-4793-4896-a8e2-be57a8bb0f50")
                IApplicationViewInteropStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetApplicationViewIdForWindow(
                        ABI::Windows::UI::Core::ICoreWindow* window,
                        INT32* id
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewInteropStatics = __uuidof(IApplicationViewInteropStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewScaling
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewScaling
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewScaling[] = L"Windows.UI.ViewManagement.IApplicationViewScaling";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("1d0ddc23-23f3-4b2d-84fe-74bf37b48b66")
                IApplicationViewScaling : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IApplicationViewScaling = __uuidof(IApplicationViewScaling);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewScalingStatics
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewScaling
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewScalingStatics[] = L"Windows.UI.ViewManagement.IApplicationViewScalingStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("b08fecf0-b946-45c8-a5e3-71f5aa78f861")
                IApplicationViewScalingStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisableLayoutScaling(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetDisableLayoutScaling(
                        boolean disableLayoutScaling,
                        boolean* success
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewScalingStatics = __uuidof(IApplicationViewScalingStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics[] = L"Windows.UI.ViewManagement.IApplicationViewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("010a6306-c433-44e5-a9f2-bd84d4030a95")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("ApplicationView static methods may be altered or unavailable for releases after Windows 8.1. Instead, use ApplicationView.GetForCurrentView to get an instance of ApplicationView.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IApplicationViewStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Value may be altered or unavailable for releases after Windows 8.1. Instead, query for window layout sizes directly.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        ABI::Windows::UI::ViewManagement::ApplicationViewState* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("TryUnsnap may be altered or unavailable for releases after Windows 8.1. Apps can be continuously resized, but cannot be snapped, starting in Windows 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE TryUnsnap(
                        boolean* success
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewStatics = __uuidof(IApplicationViewStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics2[] = L"Windows.UI.ViewManagement.IApplicationViewStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("af338ae5-cf64-423c-85e5-f3e72448fb23")
                IApplicationViewStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ViewManagement::IApplicationView** current
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TerminateAppOnFinalViewClose(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TerminateAppOnFinalViewClose(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewStatics2 = __uuidof(IApplicationViewStatics2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics3[] = L"Windows.UI.ViewManagement.IApplicationViewStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("a28d7594-8c41-4e13-9719-5164796fe4c7")
                IApplicationViewStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredLaunchWindowingMode(
                        ABI::Windows::UI::ViewManagement::ApplicationViewWindowingMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PreferredLaunchWindowingMode(
                        ABI::Windows::UI::ViewManagement::ApplicationViewWindowingMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PreferredLaunchViewSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PreferredLaunchViewSize(
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewStatics3 = __uuidof(IApplicationViewStatics3);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics4[] = L"Windows.UI.ViewManagement.IApplicationViewStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("08fd8d33-2611-5336-a315-d98e6366c9db")
                IApplicationViewStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ClearAllPersistedState(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearPersistedState(
                        HSTRING key
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewStatics4 = __uuidof(IApplicationViewStatics4);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("975f2f1e-e656-4c5e-a0a1-717c6ffa7d64")
                IApplicationViewSwitcherStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DisableShowingMainViewOnActivation(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsStandaloneAsync(
                        INT32 viewId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsStandaloneWithSizePreferenceAsync(
                        INT32 viewId,
                        ABI::Windows::UI::ViewManagement::ViewSizePreference sizePreference,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(
                        INT32 viewId,
                        ABI::Windows::UI::ViewManagement::ViewSizePreference sizePreference,
                        INT32 anchorViewId,
                        ABI::Windows::UI::ViewManagement::ViewSizePreference anchorSizePreference,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SwitchAsync(
                        INT32 viewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SwitchFromViewAsync(
                        INT32 toViewId,
                        INT32 fromViewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SwitchFromViewWithOptionsAsync(
                        INT32 toViewId,
                        INT32 fromViewId,
                        ABI::Windows::UI::ViewManagement::ApplicationViewSwitchingOptions options,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PrepareForCustomAnimatedSwitchAsync(
                        INT32 toViewId,
                        INT32 fromViewId,
                        ABI::Windows::UI::ViewManagement::ApplicationViewSwitchingOptions options,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewSwitcherStatics = __uuidof(IApplicationViewSwitcherStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics2[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("60e995cd-4fc2-48c4-b8e3-395f2b9f0fc1")
                IApplicationViewSwitcherStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DisableSystemViewActivationPolicy(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewSwitcherStatics2 = __uuidof(IApplicationViewSwitcherStatics2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics3[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("92059420-80a7-486d-b21f-c7a4a242a383")
                IApplicationViewSwitcherStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsViewModeAsync(
                        INT32 viewId,
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode viewMode,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryShowAsViewModeWithPreferencesAsync(
                        INT32 viewId,
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode viewMode,
                        ABI::Windows::UI::ViewManagement::IViewModePreferences* viewModePreferences,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewSwitcherStatics3 = __uuidof(IApplicationViewSwitcherStatics3);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTitleBar[] = L"Windows.UI.ViewManagement.IApplicationViewTitleBar";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("00924ac0-932b-4a6b-9c4b-dc38c82478ce")
                IApplicationViewTitleBar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonHoverForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonHoverForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonHoverBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonHoverBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonPressedForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonPressedForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonPressedBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonPressedBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_InactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonInactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonInactiveForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ButtonInactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ButtonInactiveBackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewTitleBar = __uuidof(IApplicationViewTitleBar);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTransferContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTransferContext[] = L"Windows.UI.ViewManagement.IApplicationViewTransferContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("8574bc63-3c17-408e-9408-8a1a9ea81bfa")
                IApplicationViewTransferContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewId(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewId(
                        INT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewTransferContext = __uuidof(IApplicationViewTransferContext);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTransferContextStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTransferContextStatics[] = L"Windows.UI.ViewManagement.IApplicationViewTransferContextStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("15a89d92-dd79-4b0b-bc47-d5f195f14661")
                IApplicationViewTransferContextStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DataPackageFormatId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewTransferContextStatics = __uuidof(IApplicationViewTransferContextStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewWithContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewWithContext[] = L"Windows.UI.ViewManagement.IApplicationViewWithContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("bd55d512-9dc1-44fc-8501-666625df60dc")
                IApplicationViewWithContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UIContext(
                        ABI::Windows::UI::IUIContext** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IApplicationViewWithContext = __uuidof(IApplicationViewWithContext);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPane[] = L"Windows.UI.ViewManagement.IInputPane";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("640ada70-06f3-4c87-a678-9829c9127c28")
                IInputPane : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Showing(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Showing(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Hiding(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Hiding(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OccludedRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPane = __uuidof(IInputPane);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPane2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPane2[] = L"Windows.UI.ViewManagement.IInputPane2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("8a6b3f26-7090-4793-944c-c3f2cde26276")
                IInputPane2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryShow(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryHide(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPane2 = __uuidof(IInputPane2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPane2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneControl[] = L"Windows.UI.ViewManagement.IInputPaneControl";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("088bb24f-962f-489d-aa6e-c6be1a0a6e52")
                IInputPaneControl : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Visible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Visible(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPaneControl = __uuidof(IInputPaneControl);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneStatics[] = L"Windows.UI.ViewManagement.IInputPaneStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("95f4af3a-ef47-424a-9741-fd2815eba2bd")
                IInputPaneStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ViewManagement::IInputPane** inputPane
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPaneStatics = __uuidof(IInputPaneStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneStatics2[] = L"Windows.UI.ViewManagement.IInputPaneStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("1b63529b-d9ec-4531-8445-71bab9fb828e")
                IInputPaneStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUIContext(
                        ABI::Windows::UI::IUIContext* context,
                        ABI::Windows::UI::ViewManagement::IInputPane** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPaneStatics2 = __uuidof(IInputPaneStatics2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPaneVisibilityEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneVisibilityEventArgs[] = L"Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("d243e016-d907-4fcc-bb8d-f77baa5028f1")
                IInputPaneVisibilityEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OccludedRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EnsuredFocusedElementInView(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EnsuredFocusedElementInView(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IInputPaneVisibilityEventArgs = __uuidof(IInputPaneVisibilityEventArgs);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IProjectionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ProjectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IProjectionManagerStatics[] = L"Windows.UI.ViewManagement.IProjectionManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("b65f913d-e2f0-4ffd-ba95-34241647e45c")
                IProjectionManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE StartProjectingAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SwapDisplaysForViewsAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopProjectingAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProjectionDisplayAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ProjectionDisplayAvailableChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ProjectionDisplayAvailableChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProjectionManagerStatics = __uuidof(IProjectionManagerStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IProjectionManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ProjectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IProjectionManagerStatics2[] = L"Windows.UI.ViewManagement.IProjectionManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("f33d2f43-2749-4cde-b977-c0c41e7415d1")
                IProjectionManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE StartProjectingWithDeviceInfoAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* displayDeviceInfo,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStartProjectingAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStartProjectingWithPlacementAsync(
                        INT32 projectionViewId,
                        INT32 anchorViewId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement prefferedPlacement,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* selector
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProjectionManagerStatics2 = __uuidof(IProjectionManagerStatics2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBar
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBar
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBar[] = L"Windows.UI.ViewManagement.IStatusBar";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("0ffcc5bf-98d0-4864-b1e8-b3f4020be8b4")
                IStatusBar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        ABI::Windows::Foundation::IAsyncAction** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE HideAsync(
                        ABI::Windows::Foundation::IAsyncAction** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundOpacity(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundOpacity(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ForegroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                        __FIReference_1_Windows__CUI__CColor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProgressIndicator(
                        ABI::Windows::UI::ViewManagement::IStatusBarProgressIndicator** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OccludedRect(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Showing(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* eventHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Showing(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Hiding(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* eventHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Hiding(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStatusBar = __uuidof(IStatusBar);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBarProgressIndicator
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBarProgressIndicator
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBarProgressIndicator[] = L"Windows.UI.ViewManagement.IStatusBarProgressIndicator";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("76cb2670-a3d7-49cf-8200-4f3eedca27bb")
                IStatusBarProgressIndicator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        ABI::Windows::Foundation::IAsyncAction** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE HideAsync(
                        ABI::Windows::Foundation::IAsyncAction** returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProgressValue(
                        __FIReference_1_double** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProgressValue(
                        __FIReference_1_double* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStatusBarProgressIndicator = __uuidof(IStatusBarProgressIndicator);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBarStatics
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBar
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBarStatics[] = L"Windows.UI.ViewManagement.IStatusBarStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("8b463fdf-422f-4561-8806-fb1289cadfb7")
                IStatusBarStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ViewManagement::IStatusBar** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStatusBarStatics = __uuidof(IStatusBarStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings[] = L"Windows.UI.ViewManagement.IUISettings";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("85361600-1c63-4627-bcb1-3a89e0bc9c55")
                IUISettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HandPreference(
                        ABI::Windows::UI::ViewManagement::HandPreference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CursorSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScrollBarSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScrollBarArrowSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScrollBarThumbBoxSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageDuration(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AnimationsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CaretBrowsingEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CaretBlinkRate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CaretWidth(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DoubleClickTime(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MouseHoverTime(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UIElementColor(
                        ABI::Windows::UI::ViewManagement::UIElementType desiredElement,
                        ABI::Windows::UI::Color* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings = __uuidof(IUISettings);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings2[] = L"Windows.UI.ViewManagement.IUISettings2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("bad82401-2721-44f9-bb91-2bb228be442f")
                IUISettings2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TextScaleFactor(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TextScaleFactorChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TextScaleFactorChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings2 = __uuidof(IUISettings2);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings3[] = L"Windows.UI.ViewManagement.IUISettings3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("03021be4-5254-4781-8194-5168f7d06d7b")
                IUISettings3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetColorValue(
                        ABI::Windows::UI::ViewManagement::UIColorType desiredColor,
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ColorValuesChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ColorValuesChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings3 = __uuidof(IUISettings3);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings4[] = L"Windows.UI.ViewManagement.IUISettings4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("52bb3002-919b-4d6b-9b78-8dd66ff4b93b")
                IUISettings4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AdvancedEffectsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AdvancedEffectsEnabledChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AdvancedEffectsEnabledChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings4 = __uuidof(IUISettings4);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings5[] = L"Windows.UI.ViewManagement.IUISettings5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("5349d588-0cb5-5f05-bd34-706b3231f0bd")
                IUISettings5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AutoHideScrollBars(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AutoHideScrollBarsChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AutoHideScrollBarsChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings5 = __uuidof(IUISettings5);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings5;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings6[] = L"Windows.UI.ViewManagement.IUISettings6";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("aef19bd7-fe31-5a04-ada4-469aaec6dfa9")
                IUISettings6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AnimationsEnabledChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AnimationsEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MessageDurationChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageDurationChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUISettings6 = __uuidof(IUISettings6);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings6;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsAnimationsEnabledChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("0c7b4b3d-2ea1-533e-894d-415bc5243c29")
                IUISettingsAnimationsEnabledChangedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IUISettingsAnimationsEnabledChangedEventArgs = __uuidof(IUISettingsAnimationsEnabledChangedEventArgs);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsAutoHideScrollBarsChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("87afd4b2-9146-5f02-8f6b-06d454174c0f")
                IUISettingsAutoHideScrollBarsChangedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IUISettingsAutoHideScrollBarsChangedEventArgs = __uuidof(IUISettingsAutoHideScrollBarsChangedEventArgs);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsMessageDurationChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("338aad52-4a5d-5b59-8002-d930f608fd6e")
                IUISettingsMessageDurationChangedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IUISettingsMessageDurationChangedEventArgs = __uuidof(IUISettingsMessageDurationChangedEventArgs);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettings[] = L"Windows.UI.ViewManagement.IUIViewSettings";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("c63657f6-8850-470d-88f8-455e16ea2c26")
                IUIViewSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserInteractionMode(
                        ABI::Windows::UI::ViewManagement::UserInteractionMode* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUIViewSettings = __uuidof(IUIViewSettings);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettingsPreferredInteractionMode[] = L"Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("426de261-82ec-5f61-bad4-19b2d0d3cf35")
                IUIViewSettingsPreferredInteractionMode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPreferredInteractionMode(
                        UINT32 supportedModesLength,
                        ABI::Windows::UI::ViewManagement::UserInteractionMode* supportedModes,
                        ABI::Windows::UI::ViewManagement::UserInteractionMode* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PreferredInteractionModeChanged(
                        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PreferredInteractionModeChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUIViewSettingsPreferredInteractionMode = __uuidof(IUIViewSettingsPreferredInteractionMode);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettingsStatics[] = L"Windows.UI.ViewManagement.IUIViewSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("595c97a5-f8f6-41cf-b0fb-aacdb81fd5f6")
                IUIViewSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ViewManagement::IUIViewSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUIViewSettingsStatics = __uuidof(IUIViewSettingsStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IViewModePreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ViewModePreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IViewModePreferences[] = L"Windows.UI.ViewManagement.IViewModePreferences";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("878fcd3a-0b99-42c9-84d0-d3f1d403554b")
                IViewModePreferences : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ViewSizePreference(
                        ABI::Windows::UI::ViewManagement::ViewSizePreference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewSizePreference(
                        ABI::Windows::UI::ViewManagement::ViewSizePreference value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CustomSize(
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IViewModePreferences = __uuidof(IViewModePreferences);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IViewModePreferencesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ViewModePreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IViewModePreferencesStatics[] = L"Windows.UI.ViewManagement.IViewModePreferencesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                MIDL_INTERFACE("69b60a65-5de5-40d8-8306-3833df7a2274")
                IViewModePreferencesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDefault(
                        ABI::Windows::UI::ViewManagement::ApplicationViewMode mode,
                        ABI::Windows::UI::ViewManagement::IViewModePreferences** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IViewModePreferencesStatics = __uuidof(IViewModePreferencesStatics);
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.ViewManagement.AccessibilitySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IAccessibilitySettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_AccessibilitySettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_AccessibilitySettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_AccessibilitySettings[] = L"Windows.UI.ViewManagement.AccessibilitySettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ActivationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IActivationViewSwitcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ActivationViewSwitcher_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ActivationViewSwitcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ActivationViewSwitcher[] = L"Windows.UI.ViewManagement.ActivationViewSwitcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewInteropStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewFullscreenStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics4 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationView ** Default Interface **
 *    Windows.UI.ViewManagement.IApplicationView2
 *    Windows.UI.ViewManagement.IApplicationView3
 *    Windows.UI.ViewManagement.IApplicationView4
 *    Windows.UI.ViewManagement.IApplicationView7
 *    Windows.UI.ViewManagement.IApplicationView9
 *    Windows.UI.ViewManagement.IApplicationViewWithContext
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationView[] = L"Windows.UI.ViewManagement.ApplicationView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs ** Default Interface **
 *    Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs[] = L"Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewScaling
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewScalingStatics interface starting with version 1.0 of the Windows.UI.ViewManagement.ViewManagementViewScalingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewScaling ** Default Interface **
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewScaling_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewScaling_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewScaling[] = L"Windows.UI.ViewManagement.ApplicationViewScaling";
#endif
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewSwitcher_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewSwitcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewSwitcher[] = L"Windows.UI.ViewManagement.ApplicationViewSwitcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTitleBar_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewTitleBar[] = L"Windows.UI.ViewManagement.ApplicationViewTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewTransferContextStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewTransferContext ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTransferContext_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTransferContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewTransferContext[] = L"Windows.UI.ViewManagement.ApplicationViewTransferContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.ViewManagement.InputPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IInputPaneStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IInputPaneStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IInputPane ** Default Interface **
 *    Windows.UI.ViewManagement.IInputPane2
 *    Windows.UI.ViewManagement.IInputPaneControl
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_InputPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_InputPane_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_InputPane[] = L"Windows.UI.ViewManagement.InputPane";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.InputPaneVisibilityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs[] = L"Windows.UI.ViewManagement.InputPaneVisibilityEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ProjectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IProjectionManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IProjectionManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ProjectionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ProjectionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ProjectionManager[] = L"Windows.UI.ViewManagement.ProjectionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.StatusBar
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IStatusBarStatics interface starting with version 1.0 of the Windows.Phone.PhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IStatusBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_StatusBar_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_StatusBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_StatusBar[] = L"Windows.UI.ViewManagement.StatusBar";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.StatusBarProgressIndicator
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IStatusBarProgressIndicator ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_StatusBarProgressIndicator_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_StatusBarProgressIndicator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_StatusBarProgressIndicator[] = L"Windows.UI.ViewManagement.StatusBarProgressIndicator";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.UISettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettings ** Default Interface **
 *    Windows.UI.ViewManagement.IUISettings2
 *    Windows.UI.ViewManagement.IUISettings3
 *    Windows.UI.ViewManagement.IUISettings4
 *    Windows.UI.ViewManagement.IUISettings5
 *    Windows.UI.ViewManagement.IUISettings6
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettings[] = L"Windows.UI.ViewManagement.UISettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.UIViewSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IUIViewSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUIViewSettings ** Default Interface **
 *    Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UIViewSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UIViewSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UIViewSettings[] = L"Windows.UI.ViewManagement.UIViewSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ViewModePreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IViewModePreferencesStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IViewModePreferences ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ViewModePreferences_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ViewModePreferences_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ViewModePreferences[] = L"Windows.UI.ViewManagement.ViewModePreferences";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3 __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPane __x_ABI_CWindows_CUI_CViewManagement_CIInputPane;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2 __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2 __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2 __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings __x_ABI_CWindows_CUI_CViewManagement_CIUISettings;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2 __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3 __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4 __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5 __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6 __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics;

#endif // ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__

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

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIInputPane* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettings* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettings* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettings* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettings* sender,
        __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreWindow __x_ABI_CWindows_CUI_CCore_CICoreWindow;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreWindow_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewBoundsMode __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewBoundsMode;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewOrientation __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewOrientation;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewState __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewState;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewSwitchingOptions __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewSwitchingOptions;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewWindowingMode __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewWindowingMode;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CFullScreenSystemOverlayMode __x_ABI_CWindows_CUI_CViewManagement_CFullScreenSystemOverlayMode;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CHandPreference __x_ABI_CWindows_CUI_CViewManagement_CHandPreference;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CUIColorType __x_ABI_CWindows_CUI_CViewManagement_CUIColorType;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CUIElementType __x_ABI_CWindows_CUI_CViewManagement_CUIElementType;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode;

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference;

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewBoundsMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewBoundsMode
{
    ApplicationViewBoundsMode_UseVisible = 0,
    ApplicationViewBoundsMode_UseCoreWindow = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode
{
    ApplicationViewMode_Default = 0,
    ApplicationViewMode_CompactOverlay = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewOrientation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewOrientation
{
    ApplicationViewOrientation_Landscape = 0,
    ApplicationViewOrientation_Portrait = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("ApplicationViewState may be altered or unavailable for releases after Windows 8.1. Instead, query for window layout sizes directly.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CViewManagement_CApplicationViewState
{
    ApplicationViewState_FullScreenLandscape = 0,
    ApplicationViewState_Filled = 1,
    ApplicationViewState_Snapped = 2,
    ApplicationViewState_FullScreenPortrait = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewSwitchingOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewSwitchingOptions
{
    ApplicationViewSwitchingOptions_Default = 0,
    ApplicationViewSwitchingOptions_SkipAnimation = 0x1,
    ApplicationViewSwitchingOptions_ConsolidateViews = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ApplicationViewWindowingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewWindowingMode
{
    ApplicationViewWindowingMode_Auto = 0,
    ApplicationViewWindowingMode_PreferredLaunchViewSize = 1,
    ApplicationViewWindowingMode_FullScreen = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    ApplicationViewWindowingMode_CompactOverlay = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    ApplicationViewWindowingMode_Maximized = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.FullScreenSystemOverlayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CFullScreenSystemOverlayMode
{
    FullScreenSystemOverlayMode_Standard = 0,
    FullScreenSystemOverlayMode_Minimal = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.HandPreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CHandPreference
{
    HandPreference_LeftHanded = 0,
    HandPreference_RightHanded = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ScreenCaptureDisabledBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CUI_CViewManagement_CScreenCaptureDisabledBehavior
{
    ScreenCaptureDisabledBehavior_DrawAsBlack = 0,
    ScreenCaptureDisabledBehavior_ExcludeFromCapture = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.UI.ViewManagement.UIColorType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CUIColorType
{
    UIColorType_Background = 0,
    UIColorType_Foreground = 1,
    UIColorType_AccentDark3 = 2,
    UIColorType_AccentDark2 = 3,
    UIColorType_AccentDark1 = 4,
    UIColorType_Accent = 5,
    UIColorType_AccentLight1 = 6,
    UIColorType_AccentLight2 = 7,
    UIColorType_AccentLight3 = 8,
    UIColorType_Complement = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.UIElementType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CUIElementType
{
    UIElementType_ActiveCaption = 0,
    UIElementType_Background = 1,
    UIElementType_ButtonFace = 2,
    UIElementType_ButtonText = 3,
    UIElementType_CaptionText = 4,
    UIElementType_GrayText = 5,
    UIElementType_Highlight = 6,
    UIElementType_HighlightText = 7,
    UIElementType_Hotlight = 8,
    UIElementType_InactiveCaption = 9,
    UIElementType_InactiveCaptionText = 10,
    UIElementType_Window = 11,
    UIElementType_WindowText = 12,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_AccentColor = 1000,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_TextHigh = 1001,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_TextMedium = 1002,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_TextLow = 1003,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_TextContrastWithHigh = 1004,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_NonTextHigh = 1005,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_NonTextMediumHigh = 1006,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_NonTextMedium = 1007,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_NonTextMediumLow = 1008,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_NonTextLow = 1009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_PageBackground = 1010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_PopupBackground = 1011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    UIElementType_OverlayOutsidePopup = 1012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.UserInteractionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode
{
    UserInteractionMode_Mouse = 0,
    UserInteractionMode_Touch = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ViewManagement.ViewSizePreference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference
{
    ViewSizePreference_Default = 0,
    ViewSizePreference_UseLess = 1,
    ViewSizePreference_UseHalf = 2,
    ViewSizePreference_UseMore = 3,
    ViewSizePreference_UseMinimum = 4,
    ViewSizePreference_UseNone = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    ViewSizePreference_Custom = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IAccessibilitySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.AccessibilitySettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IAccessibilitySettings[] = L"Windows.UI.ViewManagement.IAccessibilitySettings";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HighContrast)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HighContrastScheme)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* add_HighContrastChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CAccessibilitySettings_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_HighContrastChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettingsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_get_HighContrast(This, value) \
    ((This)->lpVtbl->get_HighContrast(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_get_HighContrastScheme(This, value) \
    ((This)->lpVtbl->get_HighContrastScheme(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_add_HighContrastChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_HighContrastChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_remove_HighContrastChanged(This, cookie) \
    ((This)->lpVtbl->remove_HighContrastChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIAccessibilitySettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IActivationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ActivationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IActivationViewSwitcher[] = L"Windows.UI.ViewManagement.IActivationViewSwitcher";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAsStandaloneAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        INT32 viewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ShowAsStandaloneWithSizePreferenceAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        INT32 viewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference sizePreference,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* IsViewPresentedOnActivationVirtualDesktop)(__x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher* This,
        INT32 viewId,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcherVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_ShowAsStandaloneAsync(This, viewId, operation) \
    ((This)->lpVtbl->ShowAsStandaloneAsync(This, viewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_ShowAsStandaloneWithSizePreferenceAsync(This, viewId, sizePreference, operation) \
    ((This)->lpVtbl->ShowAsStandaloneWithSizePreferenceAsync(This, viewId, sizePreference, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_IsViewPresentedOnActivationVirtualDesktop(This, viewId, value) \
    ((This)->lpVtbl->IsViewPresentedOnActivationVirtualDesktop(This, viewId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIActivationViewSwitcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView[] = L"Windows.UI.ViewManagement.IApplicationView";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewOrientation* value);
    HRESULT (STDMETHODCALLTYPE* get_AdjacentToLeftDisplayEdge)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AdjacentToRightDisplayEdge)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("To check full screen mode, use IsFullScreenMode. To check if the view is adjacent to both edges, use AdjacentToLeftDisplayEdge and AdjacentToRightDisplayEdge. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_IsFullScreen)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOnLockScreen)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsScreenCaptureEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsScreenCaptureEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* add_Consolidated)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_Windows__CUI__CViewManagement__CApplicationViewConsolidatedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Consolidated)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_AdjacentToLeftDisplayEdge(This, value) \
    ((This)->lpVtbl->get_AdjacentToLeftDisplayEdge(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_AdjacentToRightDisplayEdge(This, value) \
    ((This)->lpVtbl->get_AdjacentToRightDisplayEdge(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("To check full screen mode, use IsFullScreenMode. To check if the view is adjacent to both edges, use AdjacentToLeftDisplayEdge and AdjacentToRightDisplayEdge. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_IsFullScreen(This, value) \
    ((This)->lpVtbl->get_IsFullScreen(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_IsOnLockScreen(This, value) \
    ((This)->lpVtbl->get_IsOnLockScreen(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_IsScreenCaptureEnabled(This, value) \
    ((This)->lpVtbl->get_IsScreenCaptureEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_put_IsScreenCaptureEnabled(This, value) \
    ((This)->lpVtbl->put_IsScreenCaptureEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_add_Consolidated(This, handler, token) \
    ((This)->lpVtbl->add_Consolidated(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_remove_Consolidated(This, token) \
    ((This)->lpVtbl->remove_Consolidated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView2[] = L"Windows.UI.ViewManagement.IApplicationView2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SuppressSystemOverlays)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_SuppressSystemOverlays)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_VisibleBounds)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* add_VisibleBoundsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CApplicationView_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_VisibleBoundsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetDesiredBoundsMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewBoundsMode boundsMode,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* get_DesiredBoundsMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewBoundsMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_get_SuppressSystemOverlays(This, value) \
    ((This)->lpVtbl->get_SuppressSystemOverlays(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Use the TryEnterFullScreen method and IsFullScreenMode property instead of SuppressSystemOverlays. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_put_SuppressSystemOverlays(This, value) \
    ((This)->lpVtbl->put_SuppressSystemOverlays(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_get_VisibleBounds(This, value) \
    ((This)->lpVtbl->get_VisibleBounds(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_add_VisibleBoundsChanged(This, handler, token) \
    ((This)->lpVtbl->add_VisibleBoundsChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_remove_VisibleBoundsChanged(This, token) \
    ((This)->lpVtbl->remove_VisibleBoundsChanged(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_SetDesiredBoundsMode(This, boundsMode, success) \
    ((This)->lpVtbl->SetDesiredBoundsMode(This, boundsMode, success))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_get_DesiredBoundsMode(This, value) \
    ((This)->lpVtbl->get_DesiredBoundsMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView3[] = L"Windows.UI.ViewManagement.IApplicationView3";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TitleBar)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar** value);
    HRESULT (STDMETHODCALLTYPE* get_FullScreenSystemOverlayMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CFullScreenSystemOverlayMode* value);
    HRESULT (STDMETHODCALLTYPE* put_FullScreenSystemOverlayMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CFullScreenSystemOverlayMode value);
    HRESULT (STDMETHODCALLTYPE* get_IsFullScreenMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* TryEnterFullScreenMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* ExitFullScreenMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This);
    HRESULT (STDMETHODCALLTYPE* ShowStandardSystemOverlays)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This);
    HRESULT (STDMETHODCALLTYPE* TryResizeView)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        struct __x_ABI_CWindows_CFoundation_CSize value,
        boolean* success);
    HRESULT (STDMETHODCALLTYPE* SetPreferredMinSize)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3* This,
        struct __x_ABI_CWindows_CFoundation_CSize minSize);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_get_TitleBar(This, value) \
    ((This)->lpVtbl->get_TitleBar(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_get_FullScreenSystemOverlayMode(This, value) \
    ((This)->lpVtbl->get_FullScreenSystemOverlayMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_put_FullScreenSystemOverlayMode(This, value) \
    ((This)->lpVtbl->put_FullScreenSystemOverlayMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_get_IsFullScreenMode(This, value) \
    ((This)->lpVtbl->get_IsFullScreenMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_TryEnterFullScreenMode(This, success) \
    ((This)->lpVtbl->TryEnterFullScreenMode(This, success))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_ExitFullScreenMode(This) \
    ((This)->lpVtbl->ExitFullScreenMode(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_ShowStandardSystemOverlays(This) \
    ((This)->lpVtbl->ShowStandardSystemOverlays(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_TryResizeView(This, value, success) \
    ((This)->lpVtbl->TryResizeView(This, value, success))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_SetPreferredMinSize(This, minSize) \
    ((This)->lpVtbl->SetPreferredMinSize(This, minSize))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView4[] = L"Windows.UI.ViewManagement.IApplicationView4";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode* value);
    HRESULT (STDMETHODCALLTYPE* IsViewModeSupported)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode viewMode,
        boolean* isViewModeSupported);
    HRESULT (STDMETHODCALLTYPE* TryEnterViewModeAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode viewMode,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryEnterViewModeWithPreferencesAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode viewMode,
        __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* viewModePreferences,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryConsolidateAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_get_ViewMode(This, value) \
    ((This)->lpVtbl->get_ViewMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_IsViewModeSupported(This, viewMode, isViewModeSupported) \
    ((This)->lpVtbl->IsViewModeSupported(This, viewMode, isViewModeSupported))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_TryEnterViewModeAsync(This, viewMode, operation) \
    ((This)->lpVtbl->TryEnterViewModeAsync(This, viewMode, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_TryEnterViewModeWithPreferencesAsync(This, viewMode, viewModePreferences, operation) \
    ((This)->lpVtbl->TryEnterViewModeWithPreferencesAsync(This, viewMode, viewModePreferences, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_TryConsolidateAsync(This, operation) \
    ((This)->lpVtbl->TryConsolidateAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView7[] = L"Windows.UI.ViewManagement.IApplicationView7";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PersistedStateId)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PersistedStateId)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_get_PersistedStateId(This, value) \
    ((This)->lpVtbl->get_PersistedStateId(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_put_PersistedStateId(This, value) \
    ((This)->lpVtbl->put_PersistedStateId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationView9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationView9[] = L"Windows.UI.ViewManagement.IApplicationView9";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowingEnvironment)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIWindowingEnvironment** value);
    HRESULT (STDMETHODCALLTYPE* GetDisplayRegions)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9* This,
        __FIVectorView_1_Windows__CUI__CWindowManagement__CDisplayRegion** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_get_WindowingEnvironment(This, value) \
    ((This)->lpVtbl->get_WindowingEnvironment(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_GetDisplayRegions(This, result) \
    ((This)->lpVtbl->GetDisplayRegions(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationView9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewConsolidatedEventArgs[] = L"Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsUserInitiated)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_get_IsUserInitiated(This, value) \
    ((This)->lpVtbl->get_IsUserInitiated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewConsolidatedEventArgs2[] = L"Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAppInitiated)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_get_IsAppInitiated(This, value) \
    ((This)->lpVtbl->get_IsAppInitiated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewConsolidatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewFullscreenStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewFullscreenStatics[] = L"Windows.UI.ViewManagement.IApplicationViewFullscreenStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("IApplicationViewFullscreenStatics is deprecated after Windows 8. Please use other resize APIs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IApplicationViewFullscreenStatics is deprecated after Windows 8. Please use other resize APIs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* TryUnsnapToFullscreen)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics* This,
        boolean* success);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("IApplicationViewFullscreenStatics is deprecated after Windows 8. Please use other resize APIs.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_TryUnsnapToFullscreen(This, success) \
    ((This)->lpVtbl->TryUnsnapToFullscreen(This, success))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewFullscreenStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewInteropStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewInteropStatics[] = L"Windows.UI.ViewManagement.IApplicationViewInteropStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetApplicationViewIdForWindow)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics* This,
        __x_ABI_CWindows_CUI_CCore_CICoreWindow* window,
        INT32* id);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_GetApplicationViewIdForWindow(This, window, id) \
    ((This)->lpVtbl->GetApplicationViewIdForWindow(This, window, id))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewInteropStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewScaling
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewScaling
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewScaling[] = L"Windows.UI.ViewManagement.IApplicationViewScaling";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScaling_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewScalingStatics
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewScaling
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewScalingStatics[] = L"Windows.UI.ViewManagement.IApplicationViewScalingStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisableLayoutScaling)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* TrySetDisableLayoutScaling)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics* This,
        boolean disableLayoutScaling,
        boolean* success);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_get_DisableLayoutScaling(This, value) \
    ((This)->lpVtbl->get_DisableLayoutScaling(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_TrySetDisableLayoutScaling(This, disableLayoutScaling, success) \
    ((This)->lpVtbl->TrySetDisableLayoutScaling(This, disableLayoutScaling, success))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewScalingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics[] = L"Windows.UI.ViewManagement.IApplicationViewStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("ApplicationView static methods may be altered or unavailable for releases after Windows 8.1. Instead, use ApplicationView.GetForCurrentView to get an instance of ApplicationView.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Value may be altered or unavailable for releases after Windows 8.1. Instead, query for window layout sizes directly.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewState* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TryUnsnap may be altered or unavailable for releases after Windows 8.1. Apps can be continuously resized, but cannot be snapped, starting in Windows 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* TryUnsnap)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics* This,
        boolean* success);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Value may be altered or unavailable for releases after Windows 8.1. Instead, query for window layout sizes directly.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("TryUnsnap may be altered or unavailable for releases after Windows 8.1. Apps can be continuously resized, but cannot be snapped, starting in Windows 8.1.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_TryUnsnap(This, success) \
    ((This)->lpVtbl->TryUnsnap(This, success))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics2[] = L"Windows.UI.ViewManagement.IApplicationViewStatics2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIApplicationView** current);
    HRESULT (STDMETHODCALLTYPE* get_TerminateAppOnFinalViewClose)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_TerminateAppOnFinalViewClose)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_GetForCurrentView(This, current) \
    ((This)->lpVtbl->GetForCurrentView(This, current))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_get_TerminateAppOnFinalViewClose(This, value) \
    ((This)->lpVtbl->get_TerminateAppOnFinalViewClose(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_put_TerminateAppOnFinalViewClose(This, value) \
    ((This)->lpVtbl->put_TerminateAppOnFinalViewClose(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics3[] = L"Windows.UI.ViewManagement.IApplicationViewStatics3";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PreferredLaunchWindowingMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewWindowingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_PreferredLaunchWindowingMode)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewWindowingMode value);
    HRESULT (STDMETHODCALLTYPE* get_PreferredLaunchViewSize)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* put_PreferredLaunchViewSize)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_get_PreferredLaunchWindowingMode(This, value) \
    ((This)->lpVtbl->get_PreferredLaunchWindowingMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_put_PreferredLaunchWindowingMode(This, value) \
    ((This)->lpVtbl->put_PreferredLaunchWindowingMode(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_get_PreferredLaunchViewSize(This, value) \
    ((This)->lpVtbl->get_PreferredLaunchViewSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_put_PreferredLaunchViewSize(This, value) \
    ((This)->lpVtbl->put_PreferredLaunchViewSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewStatics4[] = L"Windows.UI.ViewManagement.IApplicationViewStatics4";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ClearAllPersistedState)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This);
    HRESULT (STDMETHODCALLTYPE* ClearPersistedState)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4* This,
        HSTRING key);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_ClearAllPersistedState(This) \
    ((This)->lpVtbl->ClearAllPersistedState(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_ClearPersistedState(This, key) \
    ((This)->lpVtbl->ClearPersistedState(This, key))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DisableShowingMainViewOnActivation)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This);
    HRESULT (STDMETHODCALLTYPE* TryShowAsStandaloneAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 viewId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryShowAsStandaloneWithSizePreferenceAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 viewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference sizePreference,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 viewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference sizePreference,
        INT32 anchorViewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference anchorSizePreference,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* SwitchAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 viewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SwitchFromViewAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 toViewId,
        INT32 fromViewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SwitchFromViewWithOptionsAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 toViewId,
        INT32 fromViewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewSwitchingOptions options,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* PrepareForCustomAnimatedSwitchAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics* This,
        INT32 toViewId,
        INT32 fromViewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewSwitchingOptions options,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_DisableShowingMainViewOnActivation(This) \
    ((This)->lpVtbl->DisableShowingMainViewOnActivation(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_TryShowAsStandaloneAsync(This, viewId, operation) \
    ((This)->lpVtbl->TryShowAsStandaloneAsync(This, viewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_TryShowAsStandaloneWithSizePreferenceAsync(This, viewId, sizePreference, operation) \
    ((This)->lpVtbl->TryShowAsStandaloneWithSizePreferenceAsync(This, viewId, sizePreference, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(This, viewId, sizePreference, anchorViewId, anchorSizePreference, operation) \
    ((This)->lpVtbl->TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(This, viewId, sizePreference, anchorViewId, anchorSizePreference, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_SwitchAsync(This, viewId, operation) \
    ((This)->lpVtbl->SwitchAsync(This, viewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_SwitchFromViewAsync(This, toViewId, fromViewId, operation) \
    ((This)->lpVtbl->SwitchFromViewAsync(This, toViewId, fromViewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_SwitchFromViewWithOptionsAsync(This, toViewId, fromViewId, options, operation) \
    ((This)->lpVtbl->SwitchFromViewWithOptionsAsync(This, toViewId, fromViewId, options, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_PrepareForCustomAnimatedSwitchAsync(This, toViewId, fromViewId, options, operation) \
    ((This)->lpVtbl->PrepareForCustomAnimatedSwitchAsync(This, toViewId, fromViewId, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics2[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DisableSystemViewActivationPolicy)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_DisableSystemViewActivationPolicy(This) \
    ((This)->lpVtbl->DisableSystemViewActivationPolicy(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewSwitcherStatics3[] = L"Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryShowAsViewModeAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        INT32 viewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode viewMode,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TryShowAsViewModeWithPreferencesAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3* This,
        INT32 viewId,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode viewMode,
        __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* viewModePreferences,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_TryShowAsViewModeAsync(This, viewId, viewMode, operation) \
    ((This)->lpVtbl->TryShowAsViewModeAsync(This, viewId, viewMode, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_TryShowAsViewModeWithPreferencesAsync(This, viewId, viewMode, viewModePreferences, operation) \
    ((This)->lpVtbl->TryShowAsViewModeWithPreferencesAsync(This, viewId, viewMode, viewModePreferences, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewSwitcherStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTitleBar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTitleBar[] = L"Windows.UI.ViewManagement.IApplicationViewTitleBar";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonHoverForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonHoverForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonHoverBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonHoverBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonPressedForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonPressedForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonPressedBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonPressedBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_InactiveForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_InactiveForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_InactiveBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_InactiveBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonInactiveForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonInactiveForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ButtonInactiveBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ButtonInactiveBackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar* This,
        __FIReference_1_Windows__CUI__CColor** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBarVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ForegroundColor(This, value) \
    ((This)->lpVtbl->put_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonHoverForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonHoverForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonHoverForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonHoverForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonHoverBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonHoverBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonHoverBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonHoverBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonPressedForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonPressedForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonPressedForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonPressedForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonPressedBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonPressedBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonPressedBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonPressedBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_InactiveForegroundColor(This, value) \
    ((This)->lpVtbl->put_InactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_InactiveForegroundColor(This, value) \
    ((This)->lpVtbl->get_InactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_InactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->put_InactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_InactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->get_InactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonInactiveForegroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonInactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonInactiveForegroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonInactiveForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_put_ButtonInactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->put_ButtonInactiveBackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_get_ButtonInactiveBackgroundColor(This, value) \
    ((This)->lpVtbl->get_ButtonInactiveBackgroundColor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTitleBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTransferContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTransferContext[] = L"Windows.UI.ViewManagement.IApplicationViewTransferContext";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewId)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewId)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_get_ViewId(This, value) \
    ((This)->lpVtbl->get_ViewId(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_put_ViewId(This, value) \
    ((This)->lpVtbl->put_ViewId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewTransferContextStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewTransferContextStatics[] = L"Windows.UI.ViewManagement.IApplicationViewTransferContextStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataPackageFormatId)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_get_DataPackageFormatId(This, value) \
    ((This)->lpVtbl->get_DataPackageFormatId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewTransferContextStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.ViewManagement.IApplicationViewWithContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ApplicationView
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IApplicationViewWithContext[] = L"Windows.UI.ViewManagement.IApplicationViewWithContext";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UIContext)(__x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext* This,
        __x_ABI_CWindows_CUI_CIUIContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContextVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_get_UIContext(This, value) \
    ((This)->lpVtbl->get_UIContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIApplicationViewWithContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPane[] = L"Windows.UI.ViewManagement.IInputPane";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Showing)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Showing)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Hiding)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CInputPane_Windows__CUI__CViewManagement__CInputPaneVisibilityEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Hiding)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_OccludedRect)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPane
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_add_Showing(This, handler, token) \
    ((This)->lpVtbl->add_Showing(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_remove_Showing(This, token) \
    ((This)->lpVtbl->remove_Showing(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_add_Hiding(This, handler, token) \
    ((This)->lpVtbl->add_Hiding(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_remove_Hiding(This, token) \
    ((This)->lpVtbl->remove_Hiding(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane_get_OccludedRect(This, value) \
    ((This)->lpVtbl->get_OccludedRect(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPane2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPane2[] = L"Windows.UI.ViewManagement.IInputPane2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryShow)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TryHide)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPane2* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_TryShow(This, result) \
    ((This)->lpVtbl->TryShow(This, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_TryHide(This, result) \
    ((This)->lpVtbl->TryHide(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPane2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPane2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneControl
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneControl[] = L"Windows.UI.ViewManagement.IInputPaneControl";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Visible)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Visible)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControlVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_get_Visible(This, value) \
    ((This)->lpVtbl->get_Visible(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_put_Visible(This, value) \
    ((This)->lpVtbl->put_Visible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneStatics[] = L"Windows.UI.ViewManagement.IInputPaneStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIInputPane** inputPane);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_GetForCurrentView(This, inputPane) \
    ((This)->lpVtbl->GetForCurrentView(This, inputPane))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneStatics2[] = L"Windows.UI.ViewManagement.IInputPaneStatics2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUIContext)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2* This,
        __x_ABI_CWindows_CUI_CIUIContext* context,
        __x_ABI_CWindows_CUI_CViewManagement_CIInputPane** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_GetForUIContext(This, context, result) \
    ((This)->lpVtbl->GetForUIContext(This, context, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.InputPaneVisibilityEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IInputPaneVisibilityEventArgs[] = L"Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OccludedRect)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_EnsuredFocusedElementInView)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_EnsuredFocusedElementInView)(__x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_get_OccludedRect(This, value) \
    ((This)->lpVtbl->get_OccludedRect(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_put_EnsuredFocusedElementInView(This, value) \
    ((This)->lpVtbl->put_EnsuredFocusedElementInView(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_get_EnsuredFocusedElementInView(This, value) \
    ((This)->lpVtbl->get_EnsuredFocusedElementInView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIInputPaneVisibilityEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IProjectionManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ProjectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IProjectionManagerStatics[] = L"Windows.UI.ViewManagement.IProjectionManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartProjectingAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SwapDisplaysForViewsAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* StopProjectingAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_ProjectionDisplayAvailable)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_ProjectionDisplayAvailableChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ProjectionDisplayAvailableChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_StartProjectingAsync(This, projectionViewId, anchorViewId, operation) \
    ((This)->lpVtbl->StartProjectingAsync(This, projectionViewId, anchorViewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_SwapDisplaysForViewsAsync(This, projectionViewId, anchorViewId, operation) \
    ((This)->lpVtbl->SwapDisplaysForViewsAsync(This, projectionViewId, anchorViewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_StopProjectingAsync(This, projectionViewId, anchorViewId, operation) \
    ((This)->lpVtbl->StopProjectingAsync(This, projectionViewId, anchorViewId, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_get_ProjectionDisplayAvailable(This, value) \
    ((This)->lpVtbl->get_ProjectionDisplayAvailable(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_add_ProjectionDisplayAvailableChanged(This, handler, token) \
    ((This)->lpVtbl->add_ProjectionDisplayAvailableChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_remove_ProjectionDisplayAvailableChanged(This, token) \
    ((This)->lpVtbl->remove_ProjectionDisplayAvailableChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IProjectionManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ProjectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IProjectionManagerStatics2[] = L"Windows.UI.ViewManagement.IProjectionManagerStatics2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartProjectingWithDeviceInfoAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* displayDeviceInfo,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RequestStartProjectingAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestStartProjectingWithPlacementAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        INT32 projectionViewId,
        INT32 anchorViewId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement prefferedPlacement,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2* This,
        HSTRING* selector);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_StartProjectingWithDeviceInfoAsync(This, projectionViewId, anchorViewId, displayDeviceInfo, operation) \
    ((This)->lpVtbl->StartProjectingWithDeviceInfoAsync(This, projectionViewId, anchorViewId, displayDeviceInfo, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_RequestStartProjectingAsync(This, projectionViewId, anchorViewId, selection, operation) \
    ((This)->lpVtbl->RequestStartProjectingAsync(This, projectionViewId, anchorViewId, selection, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_RequestStartProjectingWithPlacementAsync(This, projectionViewId, anchorViewId, selection, prefferedPlacement, operation) \
    ((This)->lpVtbl->RequestStartProjectingWithPlacementAsync(This, projectionViewId, anchorViewId, selection, prefferedPlacement, operation))

#define __x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_GetDeviceSelector(This, selector) \
    ((This)->lpVtbl->GetDeviceSelector(This, selector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIProjectionManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBar
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBar
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBar[] = L"Windows.UI.ViewManagement.IStatusBar";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** returnValue);
    HRESULT (STDMETHODCALLTYPE* HideAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** returnValue);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundOpacity)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundOpacity)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FIReference_1_Windows__CUI__CColor** value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FIReference_1_Windows__CUI__CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ProgressIndicator)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator** value);
    HRESULT (STDMETHODCALLTYPE* get_OccludedRect)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* add_Showing)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Showing)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Hiding)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CStatusBar_IInspectable* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Hiding)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBar* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_ShowAsync(This, returnValue) \
    ((This)->lpVtbl->ShowAsync(This, returnValue))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_HideAsync(This, returnValue) \
    ((This)->lpVtbl->HideAsync(This, returnValue))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_get_BackgroundOpacity(This, value) \
    ((This)->lpVtbl->get_BackgroundOpacity(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_put_BackgroundOpacity(This, value) \
    ((This)->lpVtbl->put_BackgroundOpacity(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_put_ForegroundColor(This, value) \
    ((This)->lpVtbl->put_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_get_ProgressIndicator(This, value) \
    ((This)->lpVtbl->get_ProgressIndicator(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_get_OccludedRect(This, value) \
    ((This)->lpVtbl->get_OccludedRect(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_add_Showing(This, eventHandler, token) \
    ((This)->lpVtbl->add_Showing(This, eventHandler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_remove_Showing(This, token) \
    ((This)->lpVtbl->remove_Showing(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_add_Hiding(This, eventHandler, token) \
    ((This)->lpVtbl->add_Hiding(This, eventHandler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_remove_Hiding(This, token) \
    ((This)->lpVtbl->remove_Hiding(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBar;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBar_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBarProgressIndicator
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBarProgressIndicator
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBarProgressIndicator[] = L"Windows.UI.ViewManagement.IStatusBarProgressIndicator";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** returnValue);
    HRESULT (STDMETHODCALLTYPE* HideAsync)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** returnValue);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ProgressValue)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* put_ProgressValue)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator* This,
        __FIReference_1_double* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicatorVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_ShowAsync(This, returnValue) \
    ((This)->lpVtbl->ShowAsync(This, returnValue))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_HideAsync(This, returnValue) \
    ((This)->lpVtbl->HideAsync(This, returnValue))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_get_ProgressValue(This, value) \
    ((This)->lpVtbl->get_ProgressValue(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_put_ProgressValue(This, value) \
    ((This)->lpVtbl->put_ProgressValue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarProgressIndicator_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IStatusBarStatics
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.StatusBar
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IStatusBarStatics[] = L"Windows.UI.ViewManagement.IStatusBarStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIStatusBar** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIStatusBarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings[] = L"Windows.UI.ViewManagement.IUISettings";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HandPreference)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CHandPreference* value);
    HRESULT (STDMETHODCALLTYPE* get_CursorSize)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_ScrollBarSize)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_ScrollBarArrowSize)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_ScrollBarThumbBoxSize)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageDuration)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AnimationsEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CaretBrowsingEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CaretBlinkRate)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CaretWidth)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DoubleClickTime)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MouseHoverTime)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* UIElementColor)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUIElementType desiredElement,
        struct __x_ABI_CWindows_CUI_CColor* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_HandPreference(This, value) \
    ((This)->lpVtbl->get_HandPreference(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_CursorSize(This, value) \
    ((This)->lpVtbl->get_CursorSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_ScrollBarSize(This, value) \
    ((This)->lpVtbl->get_ScrollBarSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_ScrollBarArrowSize(This, value) \
    ((This)->lpVtbl->get_ScrollBarArrowSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_ScrollBarThumbBoxSize(This, value) \
    ((This)->lpVtbl->get_ScrollBarThumbBoxSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_MessageDuration(This, value) \
    ((This)->lpVtbl->get_MessageDuration(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_AnimationsEnabled(This, value) \
    ((This)->lpVtbl->get_AnimationsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_CaretBrowsingEnabled(This, value) \
    ((This)->lpVtbl->get_CaretBrowsingEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_CaretBlinkRate(This, value) \
    ((This)->lpVtbl->get_CaretBlinkRate(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_CaretWidth(This, value) \
    ((This)->lpVtbl->get_CaretWidth(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_DoubleClickTime(This, value) \
    ((This)->lpVtbl->get_DoubleClickTime(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_get_MouseHoverTime(This, value) \
    ((This)->lpVtbl->get_MouseHoverTime(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings_UIElementColor(This, desiredElement, value) \
    ((This)->lpVtbl->UIElementColor(This, desiredElement, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings2[] = L"Windows.UI.ViewManagement.IUISettings2";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextScaleFactor)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* add_TextScaleFactorChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_TextScaleFactorChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings2* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_get_TextScaleFactor(This, value) \
    ((This)->lpVtbl->get_TextScaleFactor(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_add_TextScaleFactorChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_TextScaleFactorChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_remove_TextScaleFactorChanged(This, cookie) \
    ((This)->lpVtbl->remove_TextScaleFactorChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings2;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings3[] = L"Windows.UI.ViewManagement.IUISettings3";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetColorValue)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUIColorType desiredColor,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* add_ColorValuesChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_ColorValuesChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings3* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_GetColorValue(This, desiredColor, value) \
    ((This)->lpVtbl->GetColorValue(This, desiredColor, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_add_ColorValuesChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_ColorValuesChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_remove_ColorValuesChanged(This, cookie) \
    ((This)->lpVtbl->remove_ColorValuesChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings3;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings4[] = L"Windows.UI.ViewManagement.IUISettings4";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdvancedEffectsEnabled)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_AdvancedEffectsEnabledChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_AdvancedEffectsEnabledChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings4* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_get_AdvancedEffectsEnabled(This, value) \
    ((This)->lpVtbl->get_AdvancedEffectsEnabled(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_add_AdvancedEffectsEnabledChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_AdvancedEffectsEnabledChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_remove_AdvancedEffectsEnabledChanged(This, cookie) \
    ((This)->lpVtbl->remove_AdvancedEffectsEnabledChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings4;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings5[] = L"Windows.UI.ViewManagement.IUISettings5";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutoHideScrollBars)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_AutoHideScrollBarsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAutoHideScrollBarsChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AutoHideScrollBarsChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings5* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_get_AutoHideScrollBars(This, value) \
    ((This)->lpVtbl->get_AutoHideScrollBars(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_add_AutoHideScrollBarsChanged(This, handler, token) \
    ((This)->lpVtbl->add_AutoHideScrollBarsChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_remove_AutoHideScrollBarsChanged(This, token) \
    ((This)->lpVtbl->remove_AutoHideScrollBarsChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings5;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettings6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettings6[] = L"Windows.UI.ViewManagement.IUISettings6";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AnimationsEnabledChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsAnimationsEnabledChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AnimationsEnabledChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_MessageDurationChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUISettings_Windows__CUI__CViewManagement__CUISettingsMessageDurationChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MessageDurationChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettings6* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6Vtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_add_AnimationsEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_AnimationsEnabledChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_remove_AnimationsEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_AnimationsEnabledChanged(This, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_add_MessageDurationChanged(This, handler, token) \
    ((This)->lpVtbl->add_MessageDurationChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_remove_MessageDurationChanged(This, token) \
    ((This)->lpVtbl->remove_MessageDurationChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettings6;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettings6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsAnimationsEnabledChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAnimationsEnabledChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsAutoHideScrollBarsChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsAutoHideScrollBarsChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUISettingsMessageDurationChangedEventArgs[] = L"Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUISettingsMessageDurationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettings[] = L"Windows.UI.ViewManagement.IUIViewSettings";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserInteractionMode)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_get_UserInteractionMode(This, value) \
    ((This)->lpVtbl->get_UserInteractionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettingsPreferredInteractionMode[] = L"Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPreferredInteractionMode)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        UINT32 supportedModesLength,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode* supportedModes,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUserInteractionMode* result);
    HRESULT (STDMETHODCALLTYPE* add_PreferredInteractionModeChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        __FITypedEventHandler_2_Windows__CUI__CViewManagement__CUIViewSettings_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PreferredInteractionModeChanged)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionModeVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_GetPreferredInteractionMode(This, supportedModesLength, supportedModes, result) \
    ((This)->lpVtbl->GetPreferredInteractionMode(This, supportedModesLength, supportedModes, result))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_add_PreferredInteractionModeChanged(This, handler, token) \
    ((This)->lpVtbl->add_PreferredInteractionModeChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_remove_PreferredInteractionModeChanged(This, token) \
    ((This)->lpVtbl->remove_PreferredInteractionModeChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsPreferredInteractionMode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.UI.ViewManagement.IUIViewSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.UIViewSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IUIViewSettingsStatics[] = L"Windows.UI.ViewManagement.IUIViewSettingsStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics* This,
        __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIUIViewSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ViewManagement.IViewModePreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ViewModePreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IViewModePreferences[] = L"Windows.UI.ViewManagement.IViewModePreferences";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ViewSizePreference)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewSizePreference)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CViewSizePreference value);
    HRESULT (STDMETHODCALLTYPE* get_CustomSize)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* put_CustomSize)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_get_ViewSizePreference(This, value) \
    ((This)->lpVtbl->get_ViewSizePreference(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_put_ViewSizePreference(This, value) \
    ((This)->lpVtbl->put_ViewSizePreference(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_get_CustomSize(This, value) \
    ((This)->lpVtbl->get_CustomSize(This, value))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_put_CustomSize(This, value) \
    ((This)->lpVtbl->put_CustomSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.ViewManagement.IViewModePreferencesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ViewManagement.ViewModePreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ViewManagement_IViewModePreferencesStatics[] = L"Windows.UI.ViewManagement.IViewModePreferencesStatics";
typedef struct __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDefault)(__x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CApplicationViewMode mode,
        __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferences** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_CreateDefault(This, mode, result) \
    ((This)->lpVtbl->CreateDefault(This, mode, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CViewManagement_CIViewModePreferencesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.ViewManagement.AccessibilitySettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IAccessibilitySettings ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_AccessibilitySettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_AccessibilitySettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_AccessibilitySettings[] = L"Windows.UI.ViewManagement.AccessibilitySettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ActivationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IActivationViewSwitcher ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ActivationViewSwitcher_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ActivationViewSwitcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ActivationViewSwitcher[] = L"Windows.UI.ViewManagement.ActivationViewSwitcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewInteropStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewFullscreenStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewStatics4 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationView ** Default Interface **
 *    Windows.UI.ViewManagement.IApplicationView2
 *    Windows.UI.ViewManagement.IApplicationView3
 *    Windows.UI.ViewManagement.IApplicationView4
 *    Windows.UI.ViewManagement.IApplicationView7
 *    Windows.UI.ViewManagement.IApplicationView9
 *    Windows.UI.ViewManagement.IApplicationViewWithContext
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationView_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationView_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationView[] = L"Windows.UI.ViewManagement.ApplicationView";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs ** Default Interface **
 *    Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewConsolidatedEventArgs[] = L"Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewScaling
 *
 * Introduced to Windows.UI.ViewManagement.ViewManagementViewScalingContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewScalingStatics interface starting with version 1.0 of the Windows.UI.ViewManagement.ViewManagementViewScalingContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewScaling ** Default Interface **
 *
 */
#if WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewScaling_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewScaling_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewScaling[] = L"Windows.UI.ViewManagement.ApplicationViewScaling";
#endif
#endif // WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewSwitcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewSwitcherStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewSwitcher_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewSwitcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewSwitcher[] = L"Windows.UI.ViewManagement.ApplicationViewSwitcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewTitleBar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewTitleBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTitleBar_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTitleBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewTitleBar[] = L"Windows.UI.ViewManagement.ApplicationViewTitleBar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ApplicationViewTransferContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IApplicationViewTransferContextStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IApplicationViewTransferContext ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTransferContext_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ApplicationViewTransferContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ApplicationViewTransferContext[] = L"Windows.UI.ViewManagement.ApplicationViewTransferContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.UI.ViewManagement.InputPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IInputPaneStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IInputPaneStatics2 interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IInputPane ** Default Interface **
 *    Windows.UI.ViewManagement.IInputPane2
 *    Windows.UI.ViewManagement.IInputPaneControl
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_InputPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_InputPane_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_InputPane[] = L"Windows.UI.ViewManagement.InputPane";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.InputPaneVisibilityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_InputPaneVisibilityEventArgs[] = L"Windows.UI.ViewManagement.InputPaneVisibilityEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ProjectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IProjectionManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ViewManagement.IProjectionManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ProjectionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ProjectionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ProjectionManager[] = L"Windows.UI.ViewManagement.ProjectionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.StatusBar
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IStatusBarStatics interface starting with version 1.0 of the Windows.Phone.PhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IStatusBar ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_StatusBar_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_StatusBar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_StatusBar[] = L"Windows.UI.ViewManagement.StatusBar";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.StatusBarProgressIndicator
 *
 * Introduced to Windows.Phone.PhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IStatusBarProgressIndicator ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_StatusBarProgressIndicator_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_StatusBarProgressIndicator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_StatusBarProgressIndicator[] = L"Windows.UI.ViewManagement.StatusBarProgressIndicator";
#endif
#endif // WINDOWS_PHONE_PHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.UISettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettings ** Default Interface **
 *    Windows.UI.ViewManagement.IUISettings2
 *    Windows.UI.ViewManagement.IUISettings3
 *    Windows.UI.ViewManagement.IUISettings4
 *    Windows.UI.ViewManagement.IUISettings5
 *    Windows.UI.ViewManagement.IUISettings6
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettings[] = L"Windows.UI.ViewManagement.UISettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsAnimationsEnabledChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsAutoHideScrollBarsChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UISettingsMessageDurationChangedEventArgs[] = L"Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.UI.ViewManagement.UIViewSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IUIViewSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IUIViewSettings ** Default Interface **
 *    Windows.UI.ViewManagement.IUIViewSettingsPreferredInteractionMode
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_UIViewSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_UIViewSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_UIViewSettings[] = L"Windows.UI.ViewManagement.UIViewSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ViewManagement.ViewModePreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ViewManagement.IViewModePreferencesStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ViewManagement.IViewModePreferences ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_ViewManagement_ViewModePreferences_DEFINED
#define RUNTIMECLASS_Windows_UI_ViewManagement_ViewModePreferences_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ViewManagement_ViewModePreferences[] = L"Windows.UI.ViewManagement.ViewModePreferences";
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
#endif // __windows2Eui2Eviewmanagement_p_h__

#endif // __windows2Eui2Eviewmanagement_h__
